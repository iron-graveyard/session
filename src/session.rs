//! Session-storage middleware internals.

use std::sync::{Arc, RWLock};
use std::collections::HashMap;
use collections::hash::Hash;
use core::cmp::Eq;
use iron::{Request, Response, Middleware, Alloy};
use iron::middleware::{Status, Continue};

pub type SessionStore<K, V> = Arc<RWLock<HashMap<K, V>>>;

/// The session `Middleware`.
pub struct Session<K, V> {
    session_store: Arc<RWLock<HashMap<K, V>>>
}

impl<K: Send + Share, V: Send + Share> Clone for Session<K, V> {
    fn clone(&self) -> Session<K, V> {
        Session {
            session_store: self.session_store.clone()
        }
    }
}

impl<K: Hash + Eq + Send + Share, V: Send + Share> Session<K, V> {
    /// Create a new instance of the session `Middleware`.
    pub fn new() -> Session<K, V> {
        Session {
            session_store: Arc::new(RWLock::new(HashMap::<K, V>::new()))
        }
    }
}

impl<K: Send, V: Send> Middleware for Session<K, V> {
    /// Adds the session store to the `alloy`.
    fn enter(&mut self, _: &mut Request, _: &mut Response,
             alloy: &mut Alloy) -> Status {
        alloy.insert(self.session_store.clone());
        Continue
    }
}
