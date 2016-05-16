//! This exposes `Session`, the struct stored in the `Alloy`.

use std::sync::Arc;
use super::SessionStore;
use iron::typemap;

/// A session which provides basic CRUD operations.
pub struct Session<K: typemap::Key> {
    key: K,
    store: Arc<Box<SessionStore<K> + 'static + Send + Sync>>
}

impl<K: typemap::Key> Session<K> {
    /// Create a new session
    pub fn new(key: K, store: Box<SessionStore<K> + 'static + Send + Sync>) -> Session<K> {
        Session {
            key: key,
            store: Arc::new(store)
        }
    }
    /// Set the value of this session, replacing any previously set value.
    pub fn insert(&self, value: K::Value) {
        self.store.insert(&self.key, value)
    }
    /// Retrieve the value of this session.
    ///
    /// Returns `None` if this session has not been set.
    pub fn find(&self) -> Option<K::Value> {
        self.store.find(&self.key)
    }
    /// Swap the given value with the current value of this session.
    ///
    /// Returns the value being replaced.
    /// Returns `None` if this session was not yet set.
    pub fn swap(&self, value: K::Value) -> Option<K::Value> {
        self.store.swap(&self.key, value)
    }
    /// Insert value, if not yet set, or update the current value of this session.
    ///
    /// Returns an owned copy of the set (current) value of this session.
    ///
    /// This is analagous to the `insert_or_update_with` method of `HashMap`.
    pub fn upsert(&self, value: K::Value, mutator: fn(&mut K::Value)) -> K::Value {
        self.store.upsert(&self.key, value, mutator)
    }
    /// Remove the session stored at this key.
    pub fn remove(&self) -> bool {
        self.store.remove(&self.key)
    }
}
