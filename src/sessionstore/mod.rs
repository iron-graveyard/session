//! This module defines the trait necessary for a session storage struct.
///!
///! A default implementation can be found in the `store` module: `Session`.

pub mod store;

/// This trait must be implemented for any session storage struct.
///
/// A key is set using `set_key` when the `Session` middleware is first called.
///
/// All of the remaining methods act on the specific session
/// selected by that key. This isolates sessions to a specific request.
pub trait SessionStore<K, V>: Send {
    /// Set the key to identify a unique session.
    fn set_key(&mut self, key: K);
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
