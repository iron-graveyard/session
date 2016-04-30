//! This module defines the trait necessary for a session storage struct.

use self::session::Session;
use iron::typemap;

pub mod session;

/// A default implementation of `SessionStore`: `Session`.
pub mod hashsession;

/// This `Trait` defines a session storage struct. It must be implemented on any store passed to `Sessions`.
pub trait SessionStore<K: typemap::Key>: Sync {
    #[doc(hidden)]
    fn select_session(&mut self, key: K) -> Session<K> where Self: Send + Clone + 'static {
        Session::new(key, Box::new(self.clone()))
      }
    // fn select_session(&self, key: K) -> Session<K>
    // where Self: Clone + SessionStore<K> + 'static {
    //     Session::new(key, self.clone())
    // }
    // fn select_session(&mut self, key: K) -> Session<K> {
    //     Session::new(key, self.clone()) //Box::new(self.clone()))
    // }
    /// Set the value of the session belonging to `key`, replacing any previously set value.
    fn insert(&self, key: &K, value: K::Value);
    /// Retrieve the value of this session.
    ///
    /// Returns `None` if the session belonging to `key` has not been set.
    fn find(&self, key: &K) -> Option<K::Value>;
    /// Swap the given value with the current value of the session belonging to `key`.
    ///
    /// Returns the value being replaced, or `None` if this session was not yet set.
    fn swap(&self, key: &K, value: K::Value) -> Option<K::Value>;
    /// Insert value, if not yet set, or update the current value of the session belonging to `key`.
    ///
    /// Returns an owned copy of the value that was set.
    ///
    /// This is analagous to the `insert_or_update_with` method of `HashMap`.
    fn upsert(&self, key: &K, value: K::Value, mutator: fn(&mut K::Value)) -> K::Value;
    /// Remove the session stored at this key.
    fn remove(&self, key: &K) -> bool;
}
