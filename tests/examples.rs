use std::collections::HashMap;

use michie::memoized;

#[test]
fn conditional_caching() {
    #[derive(Default)]
    pub struct Store<K,V>(HashMap<K,V>);

    impl<K, T, E> michie::MemoizationStore<K, Result<T, E>> for Store<K,Result<T,E>> where K: Eq + std::hash::Hash, {
        fn insert(&mut self, key: K, value: Result<T, E>) {
            if value.is_ok() {
                HashMap::insert(&mut self.0, key, value);
            }
        }

        fn get(&self, key: &K) -> Option<&Result<T, E>> {
            self.0.get(key)
        }
    }

    #[memoized(key_expr = input)]
    fn may_fail(input: usize) -> Result<usize, ()> {
        let remainder = input % 2;
        if remainder == 0 {
            return Err(());
        }
        Ok(remainder)
    }

    assert_eq!(may_fail(2), Err(()));
    assert_eq!(may_fail(3), Ok(1));
}
