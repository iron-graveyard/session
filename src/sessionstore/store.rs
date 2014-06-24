use std::sync::Arc;
use std::sync::RWLock;
use std::collections::HashMap;
use collections::hash::Hash;
use core::cmp::Eq;
use super::SessionStore;

type Store<K, V> = RWLock<HashMap<K, RWLock<V>>>;

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
    pub fn new() -> Session<K, V> {
        Session {
            key: None,
            store: Arc::new(RWLock::new(HashMap::<K, RWLock<V>>::new()))
        }
    }
}

impl<K: Hash + Eq + Send + Share + Clone, V: Send + Share + Clone> SessionStore<K, V> for Session<K, V> {
    fn set_key(&mut self, key: K) { self.key = Some(key); }
    fn insert(&self, val: V) {
        let key = self.key.clone().unwrap();
        // Avoid a WriteLock if possible
        if !self.store.read().contains_key(&key) {
            self.store.write().insert(key, RWLock::new(val));
        }
    }
    fn find(&self) -> Option<V> {
        let key = self.key.clone().unwrap();
        match self.store.read().find(&key) {
            Some(lock) => Some(lock.read().clone()),
            None => None
        }
    }
    fn swap(&self, new_value: V) -> Option<V> {
        let key = self.key.clone().unwrap();
        match self.store.read().find(&key) {
            Some(lock) => {
                *lock.write() = new_value;
                Some(lock.read().clone())
            },
            None       => {
                self.insert(new_value);
                None
            }
        }
    }
    fn upsert(&self, value: V, mutator: |&mut V|) -> V {
        let key = self.key.clone().unwrap();
        match self.store.read().find(&key) {
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
        let key = self.key.clone().unwrap();
        self.store.write().remove(&key)
    }
}
