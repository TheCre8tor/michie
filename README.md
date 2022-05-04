[![Version](https://img.shields.io/crates/v/michie)][crates.io]
[![License](https://img.shields.io/crates/l/michie)][license]
![Downloads](https://img.shields.io/crates/d/michie)
![Recent downloads](https://img.shields.io/crates/dr/michie)
[![CI status](https://github.com/mobusoperandi/michie/actions/workflows/ci.yml/badge.svg)][ci]

michie (pronounced /'mikɪ/) — an attribute macro that adds [memoization] to a function.

# Features

- Supports
    - Plain functions
    - Generic functions
    - Functions in `impl` blocks
    - Functions in trait implementation blocks
    - Functions that are default trait implementations
- Thread safe
- Expansion depends on only `std`
- Hygienic
- Supports recursive functions
- Bring your own store

# `key_expr`

In each invocation a key is obtained.
It is used to query the function's cache store for a possible hit.
An expression that evaluates into a key must be provided via the `key_expr` argument.
The expression may use bindings from the function's parameters.
In the following example the `key_expr` is simply the name of the only parameter.

```rust
use michie::memoized;
#[memoized(key_expr = input)]
fn f(input: usize) -> usize {
    // expensive calculation
    # unimplemented!()
}
```

# `key_type`

While the type of the key supports inference, it may also be specified using the `key_type` argument:

```rust
use michie::memoized;
#[memoized(key_type = u64, key_expr = input.into())]
fn f(input: u32) -> u32 {
    // expensive calculation
    # unimplemented!()
}
```

# `store_type`

A store type may be provided via the `store_type` argument.
The provided type must implement [`MemoizationStore`].
Implementations of [`MemoizationStore`] for [`BTreeMap`] and [`HashMap`] are provided.
In the following example, [`BTreeMap`] is provided as the store:

```rust
use michie::memoized;
use std::collections::BTreeMap;
#[memoized(key_expr = input, store_type = BTreeMap<usize, usize>)]
fn f(input: usize) -> usize {
    // expensive calculation
    # unimplemented!()
}
```

# `store_init`

By default, the store is initialized via [`Default::default()`].
Different initialization may be provided via an expression to `store_init`:

```rust
use michie::{memoized, MemoizationStore};
use std::collections::HashMap;
#[memoized(key_expr = input, store_init = HashMap::with_capacity(500))]
fn f(input: usize) -> usize {
    // expensive calculation
    # unimplemented!()
}
```

# Store inference and the default store

An omitted `store_type` _may_ be inferred from a provided `store_init`.
If both are omitted, the default `store_type` is [`HashMap`].

# Type requirements

Bounds apply to the key type and the function's return type.
Some are from the general instrumentation and others are via the store type's implementation of [`MemoizationStore`].

## General bounds

The following apply to the key type and to the function's return type:

- [`Sized`]: for one, the instrumentation stores the key in a `let` binding.
- [`'static`]: key and return values are owned by a store which is owned by a static.
- [`Clone`]: the key and return values are cloned for insertion into the store.
- [`Send`] and [`Sync`]: for parallel access.

## Store bounds

Bounds on `K` and `R` in the `store_type`'s implementation of [`MemoizationStore`] apply to the key type and the return type respectively.
The default store type, [`HashMap`], bounds `where K: Borrow<Q>, Q: Hash + Eq`.

# Generic functions

Be mindful of the [type requirements](#type-requirements) when using on a generic function:

```rust
use michie::memoized;
use std::hash::Hash;
#[memoized(key_expr = input.clone())]
fn f<A, B>(input: A) -> B
where
    A: Clone + Send + Sync + 'static + Eq + Hash,
    B: Clone + Send + Sync + 'static + From<A>,
{
    input.into()
}
```

# Functions that take no input

Functions that take no input are good candidates for [compile-time evaluation],
which is usually preferred over runtime caching (such as this crate provides).
Nonetheless, some functions cannot be evaluated at compile time.
A reasonable `key_expr` for a function that takes no input is `()`:

```rust
use michie::memoized;
#[memoized(key_expr = ())]
fn f() -> f64 {
    // expensive calculation
    # unimplemented!()
}
```

# How it works

The original function expands into something similar to this:

```rust ignore
fn f(input: Input) -> Output {
    static STORE = Mutex::new(#store_init);
    let key = #key_expr;
    if let Some(hit) = STORE.lock().unwrap().get(&key) {
        return hit;
    } else {
        let miss = #original_fn_body;
        STORE.lock().unwrap().insert(key, miss.clone());
        return miss;
    };
}
```

# Why must `key_expr` be provided?

The only conceivable default `key_expr` is the entire input.
For example, for a function signature:
```text
fn f(a: usize, _b: usize) -> usize
```
the default `key_expr` would be `(a, _b)`.
Two potential problems: some parameters might not satisfy [bounds on the key type](#type-requirements).
Also, the resulting key might be a supervalue of _the input of the_ __actual__ _calculation_.
To explain the latter problem, here is an example:

```rust
use michie::memoized;
// pretend that `key_expr` is omitted and that this is the default
#[memoized(key_expr = (a, _b))]
fn f(a: usize, _b: usize) -> usize {
    // the actual calculation uses a subvalue of the input — only `a`
    # a
}
f(0, 0); // expected miss because it's the first invocation
f(0, 1); // avoidable miss!
```

If an accurate `key_expr = a` had been provided, the second execution would have been a hit.
To summarize, `key_expr` is mandatory in order to encourage proper consideration of it.

# Support and feedback

In [the GitHub Discussions].

# Authored by Mobus Operandi

This crate is a work by [Mobus Operandi] — a community for the study of Rust in mob programming format.

[`'static`]: https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html#trait-bound
[compile-time evaluation]: https://doc.rust-lang.org/std/keyword.const.html#compile-time-evaluable-functions
[memoization]: https://en.wikipedia.org/wiki/Memoization
[Mobus Operandi]: https://github.com/mobusoperandi
[crates.io]: https://crates.io/crates/michie
[ci]: https://github.com/mobusoperandi/michie/actions/workflows/ci.yml
[license]: https://tldrlegal.com/license/mit-license
[the GitHub Discussions]: https://github.com/mobusoperandi/michie/discussions
