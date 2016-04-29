//! This module defines the trait necessary for a session storage struct.

use self::session::Session;
use std::any::Any;

pub mod session;

/// A default implementation of `SessionStore`: `Session`.
pub mod hashsession;

/// This `Trait` defines a session storage struct. It must be implemented on any store passed to `Sessions`.
pub trait SessionStore<K, V>: Sync + Send {
    #[doc(hidden)]
    // fn select_session(&self, key: K) -> Session<K> where Self: Clone + 'static, V: Any + 'static {
    //     let b = Box::new(self.clone());
    //     Session::new(key, b)
    // }
    fn select_session(&self, key: K) -> Session<K>
    where Self: Clone + SessionStore<K, Box<Any + 'static>> + 'static {
        println!("{}", "select session");
        let b = Box::new(self.clone());
        Session::new(key, b)
    }
    /// Set the value of the session belonging to `key`, replacing any previously set value.
    fn insert(&self, key: &K, value: V);
    /// Retrieve the value of this session.
    ///
    /// Returns `None` if the session belonging to `key` has not been set.
    fn find(&self, key: &K) -> Option<V>;
    /// Swap the given value with the current value of the session belonging to `key`.
    ///
    /// Returns the value being replaced, or `None` if this session was not yet set.
    fn swap(&self, key: &K, value: V) -> Option<V>;
    /// Insert value, if not yet set, or update the current value of the session belonging to `key`.
    ///
    /// Returns an owned copy of the value that was set.
    ///
    /// This is analagous to the `insert_or_update_with` method of `HashMap`.
    fn upsert(&self, key: &K, value: V, mutator: fn(&mut V)) -> V;
    /// Remove the session stored at this key.
    fn remove(&self, key: &K) -> bool;
}
