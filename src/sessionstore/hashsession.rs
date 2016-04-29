use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::hash::Hash;
use core::cmp::Eq;
use super::SessionStore;

type Store<K, V> = RwLock<HashMap<K, RwLock<V>>>;

/// A default implementation of `SessionStore`.
///
/// Session store implemented as a read-write-locked `HashMap`.
///
/// #### To use:
/// ```ignore
/// // When defining your server:
/// server.link(Sessions::new(key_gen_fn, HashSessionStore::<KeyType, ValueType>::new()));
///
/// // When accessing from your middleware:
/// let session = alloy.find_mut::<Session<KeyType, ValueType>>().unwrap();
/// ```
pub struct HashSessionStore<K, V>{
    store: Arc<Store<K, V>>
}

impl<K: Clone + Send, V: Send> Clone for HashSessionStore<K, V> {
    fn clone(&self) -> HashSessionStore<K, V> {
        HashSessionStore {
            store: self.store.clone()
        }
    }
}

impl<K: Hash + Eq + Send + Sync, V: Send + Sync> HashSessionStore<K, V> {
    /// Create a new instance of the session store
    pub fn new() -> HashSessionStore<K, V> {
        HashSessionStore {
            store: Arc::new(RwLock::new(HashMap::<K, RwLock<V>>::new()))
        }
    }
}

/* A note on clones:
 *
 * Those values hidden behind a RwLock are owned behind that lock.
 * In order for them to be accessed, a reference to the two gating locks
 * (the HashMap and the keyed V) must be kept alive.
 *
 * Instead, all values returned are copies.
 */
impl<K: Hash + Eq + Send + Sync + Clone, V: Send + Sync + Clone> SessionStore<K, V> for HashSessionStore<K, V> {
    fn insert(&self, key: &K, val: V) {
        // Avoid a WriteLock if possible
        if !self.store.read().unwrap().contains_key(key) {
            // Inserting consumes a key => clone()
            self.store.write().unwrap().insert(key.clone(), RwLock::new(val));
        }
    }
    fn find(&self, key: &K) -> Option<V> {
        match self.store.read().unwrap().get(key) {
            Some(lock) => Some(lock.read().unwrap().clone()),
            None => None
        }
    }
    fn swap(&self, key: &K, value: V) -> Option<V> {
        match self.store.read().unwrap().get(key) {
            // Instead of using swap, which requires a write lock on the HashMap,
            // only take the write locks when the key does not yet exist
            Some(lock) => {
                let old_v = lock.read().unwrap().clone();
                *lock.write().unwrap() = value;
                return Some(old_v)
            },
            None => ()
        }
        self.insert(key, value);
        None
    }
    fn upsert(&self, key: &K, value: V, mutator: fn(&mut V)) -> V {
        match self.store.read().unwrap().get(key) {
            Some(lock) => {
                let old_v = &mut *lock.write().unwrap();
                mutator(old_v);
                return old_v.clone()
            },
            None => ()
        }
        self.insert(key, value.clone());
        value
    }
    fn remove(&self, key: &K) -> bool {
        self.store.write().unwrap().remove(key).is_some()
    }
}

#[cfg(test)]
mod test {
    pub use super::*;
    pub use super::super::*;
    pub use super::super::session::*;
    pub use super::super::super::sessions::*;
    pub use iron::*;
    pub use test::mock::{request, response};

    fn dummy_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::new())
    }

    pub fn set_server() -> ChainBuilder {
        let mut chain = ChainBuilder::new(dummy_handler);
        chain.link_before(
            Sessions::new(
                get_session_id,
                HashSessionStore::<char, char>::new()
            )
        ); chain
    }

    pub fn run_server(chain: ChainBuilder) {
        let _ = Iron::new(chain).http("localhost:3000");
    }

    pub fn get_session_id(_: &Request) -> char {'a'}

    pub fn set_session_to_a(req: &mut Request) -> IronResult<()> {
        let session = req.extensions.find::<RequestSession, Session<char, char>>().unwrap();
        session.insert('a');
        Ok(())
    }
    pub fn set_session_to_b(req: &mut Request) -> IronResult<()> {
        let session = req.extensions.find::<RequestSession, Session<char, char>>().unwrap();
        session.insert('b');
        Ok(())
    }
    pub fn swap_session_to_b(req: &mut Request) -> IronResult<()> {
        let session = req.extensions.find::<RequestSession, Session<char, char>>().unwrap();
        session.swap('b');
        Ok(())
    }
    pub fn upsert_session(req: &mut Request) -> IronResult<()> {
        let session = req.extensions.find::<RequestSession, Session<char, char>>().unwrap();
        let _ = session.upsert('b', |c: &mut char| *c = 'a');
        Ok(())
    }
    pub fn remove_session(req: &mut Request) -> IronResult<()> {
        let session = req.extensions.find::<RequestSession, Session<char, char>>().unwrap();
        session.remove();
        Ok(())
    }
    pub fn check_session_is_not_set(req: &mut Request) -> IronResult<()> {
        let session = req.extensions.find::<RequestSession, Session<char, char>>().unwrap();
        assert_eq!(session.find(), None);
        Ok(())
    }
    pub fn check_session_is_set_to_a(req: &mut Request) -> IronResult<()> {
        let session = req.extensions.find::<RequestSession, Session<char, char>>().unwrap();
        assert_eq!(session.find(), Some('a'));
        Ok(())
    }
    pub fn check_session_is_set_to_b(req: &mut Request) -> IronResult<()> {
        let session = req.extensions.find::<RequestSession, Session<char, char>>().unwrap();
        assert_eq!(session.find(), Some('b'));
        Ok(())
    }

    mod enter {
        pub use super::*;

        #[test]
        fn starts_with_empty_session() {
            let mut chain = set_server();
            chain.link_before(check_session_is_not_set);
            run_server(chain);
        }

        #[test]
        fn finds_session() {
            let mut chain = set_server();
            chain.link_before(set_session_to_a);
            chain.link_before(check_session_is_set_to_a);
            run_server(chain);
        }

        mod swap {
            use super::*;

            #[test]
            fn swaps_session_when_empty() {
                let mut chain = set_server();
                chain.link_before(swap_session_to_b);
                chain.link_before(check_session_is_set_to_b);
                run_server(chain);
            }

            #[test]
            fn swaps_session_when_non_empty() {
                let mut chain = set_server();
                chain.link_before(set_session_to_a);
                chain.link_before(swap_session_to_b);
                chain.link_before(check_session_is_set_to_b);
                run_server(chain);
            }


            #[test]
            fn swaps_session_when_same_valued() {
                let mut chain = set_server();
                chain.link_before(set_session_to_b);
                chain.link_before(swap_session_to_b);
                chain.link_before(check_session_is_set_to_b);
                run_server(chain);
            }
        }

        mod upsert {
            use super::*;

            #[test]
            fn inserts_session_when_empty() {
                let mut chain = set_server();
                chain.link_before(upsert_session);
                chain.link_before(check_session_is_set_to_b);
                run_server(chain);
            }

            #[test]
            fn mutates_session_when_non_empty() {
                let mut chain = set_server();
                chain.link_before(set_session_to_b);
                chain.link_before(upsert_session);
                chain.link_before(check_session_is_set_to_a);
                run_server(chain);
            }
        }

        #[test]
        fn removes_session() {
            let mut chain = set_server();
            chain.link_before(set_session_to_a);
            chain.link_before(check_session_is_set_to_a);
            chain.link_before(remove_session);
            chain.link_before(check_session_is_not_set);
            run_server(chain);
        }
    }
}
