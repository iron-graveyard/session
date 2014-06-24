use std::sync::Arc;
use std::sync::RWLock;
use std::collections::HashMap;
use collections::hash::Hash;
use core::cmp::Eq;
use super::SessionStore;

type Store<K, V> = RWLock<HashMap<K, RWLock<V>>>;

/// A default implementation of `SessionStore`.
///
/// Session store implemented as a read-write-locked `HashMap`.
///
/// #### To use:
/// ```ignore
/// // When defining your server:
/// server.link(Sessions::new(key_gen_fn, Session::<KeyType, ValueType>::new()));
///
/// // When accessing from your middleware:
/// let session = alloy.find_mut::<Session<KeyType, ValueType>>().unwrap();
/// ```
pub struct Session<K, V>{
    key: Option<K>,
    store: Arc<Store<K, V>>
}

impl<K: Clone + Send, V: Send> Clone for Session<K, V> {
    fn clone(&self) -> Session<K, V> {
        Session {
            key: self.key.clone(),
            store: self.store.clone()
        }
    }
}

impl<K: Hash + Eq + Send + Share, V: Send + Share> Session<K, V> {
    /// Create a new instance of the session store
    pub fn new() -> Session<K, V> {
        Session {
            key: None,
            store: Arc::new(RWLock::new(HashMap::<K, RWLock<V>>::new()))
        }
    }
}

/* A note on clones:
 *
 * Those values hidden behind a RWLock are owned behind that lock.
 * In order for them to be accessed, a reference to the two gating locks
 * (the HashMap and the keyed V) must be kept alive.
 *
 * Instead, all values returned are copies.
 */
impl<K: Hash + Eq + Send + Share + Clone, V: Send + Share + Clone> SessionStore<K, V> for Session<K, V> {
    fn set_key(&mut self, key: K) { self.key = Some(key); }
    fn insert(&self, val: V) {
        let key = self.key.as_ref().unwrap();
        // Avoid a WriteLock if possible
        if !self.store.read().contains_key(key) {
            // Inserting consumes a key => clone()
            self.store.write().insert(key.clone(), RWLock::new(val));
        }
    }
    fn find(&self) -> Option<V> {
        let key = self.key.as_ref().unwrap();
        match self.store.read().find(key) {
            Some(lock) => Some(lock.read().clone()),
            None => None
        }
    }
    fn swap(&self, value: V) -> Option<V> {
        let key = self.key.as_ref().unwrap();
        match self.store.read().find(key) {
            // Instead of using swap, which requires a write lock on the HashMap,
            // only take the write locks when the key does not yet exist
            Some(lock) => {
                let old_v = lock.read().clone();
                *lock.write() = value;
                Some(old_v)
            },
            None       => {
                self.insert(value);
                None
            }
        }
    }
    fn upsert(&self, value: V, mutator: |&mut V|) -> V {
        let key = self.key.as_ref().unwrap();
        match self.store.read().find(key) {
            Some(lock) => {
                let old_v = &mut *lock.write();
                mutator(old_v);
                old_v.clone()
            },
            None => {
                self.insert(value.clone());
                value
            }
        }
    }
    fn remove(&self) -> bool {
        let key = self.key.as_ref().unwrap();
        self.store.write().remove(key)
    }
}
