use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::hash::Hash;
use core::cmp::Eq;
use super::SessionStore;
use iron::typemap;

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
pub struct HashSessionStore<K: typemap::Key>{
    store: Arc<Store<K, K::Value>>
}

impl<K: typemap::Key> Clone for HashSessionStore<K> {
    fn clone(&self) -> HashSessionStore<K> {
        HashSessionStore {
            store: self.store.clone()
        }
    }
}

impl<K: typemap::Key> HashSessionStore<K> where K: Eq + Hash {
    /// Create a new instance of the session store
    pub fn new() -> HashSessionStore<K> {
        HashSessionStore {
            store: Arc::new(RwLock::new(HashMap::<K, RwLock<K::Value>>::new()))
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
impl<K: typemap::Key> SessionStore<K> for HashSessionStore<K> where K: Send + Sync + Eq + Hash + Clone, K::Value: Send + Sync + Clone {
    fn insert(&self, key: &K, val: K::Value) {
        // Avoid a WriteLock if possible
        if !self.store.read().unwrap().contains_key(key) {
            // Inserting consumes a key => clone()
            self.store.write().unwrap().insert(key.clone(), RwLock::new(val));
        }
    }
    fn find(&self, key: &K) -> Option<K::Value> {
        match self.store.read().unwrap().get(key) {
            Some(lock) => Some(lock.read().unwrap().clone()),
            None => None
        }
    }
    fn swap(&self, key: &K, value: K::Value) -> Option<K::Value> {
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
    fn upsert(&self, key: &K, value: K::Value, mutator: fn(&mut K::Value)) -> K::Value {
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