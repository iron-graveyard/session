use std::sync::Arc;
use super::SessionStore;

pub struct Session<K, V> {
    key: K,
    arc: Arc<Box<SessionStore<K, V> + 'static>>
}

impl<K, V> Session<K, V> {
    /// Set the value of this session, replacing any previously set value.
    fn insert(&self, value: V) {
        self.arc.insert(&self.key, value)
    }
    /// Retrieve the value of this session.
    ///
    /// Returns `None` if this session has not been set.
    fn find(&self) -> Option<V> {
        self.arc.find(&self.key)
    }
    /// Swap the given value with the current value of this session.
    ///
    /// Returns the value being replaced.
    /// Returns `None` if this session was not yet set.
    fn swap(&self, value: V) -> Option<V> {
        self.arc.swap(&self.key, value)
    }
    /// Insert value, if not yet set, or update the current value of this session.
    ///
    /// Returns an owned copy of the set (current) value of this session.
    ///
    /// This is analagous to the `insert_or_update_with` method of `HashMap`.
    fn upsert(&self, value: V, mutator: |&mut V|) -> V {
        self.arc.upsert(&self.key, value, mutator)
    }
    /// Remove the session stored at this key.
    fn remove(&self) -> bool {
        self.arc.remove(&self.key)
    }
}
