//! This module defines the trait necessary for a session storage struct.

/// A default implementation of `SessionStore`: `Session`.
pub mod store;

/// This `Trait` defines a session storage struct. It must be implemented on any store passed to `Sessions`.
///
/// `select_session` is called for each request to select the session.
///
/// All of the remaining methods act on that session.
/// This isolates requests to a specific session.
pub trait SessionStore<K, V>: Send {
    /// Select the session with a key, given by the `key_generator` function.
    fn select_session(&mut self, key: K);
    /// Set the value of this session, replacing any previously set value.
    fn insert(&self, value: V);
    /// Retrieve the value of this session.
    ///
    /// Returns `None` if this session has not been set.
    fn find(&self) -> Option<V>;
    /// Swap the given value with the current value of this session.
    ///
    /// Returns the value being replaced.
    /// Returns `None` if this session was not yet set.
    fn swap(&self, value: V) -> Option<V>;
    /// Insert value, if not yet set, or update the current value of this session.
    ///
    /// Returns an owned copy of the set (current) value of this session.
    ///
    /// This is analagous to the `insert_or_update_with` method of `HashMap`.
    fn upsert(&self, value: V, mutator: |&mut V|) -> V;
    /// Remove the session stored at this key.
    fn remove(&self) -> bool;
}
