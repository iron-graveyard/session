//! Sessioning middleware
//!
//! Instantiate and link a new `Sessions` struct to
//! give your server sessioning functionality.
//!
//! Key-generating functions and custom stores can be used
//! to customize functionality.

use iron::{Request, Response, Middleware, Alloy};
use iron::middleware::{Status, Continue};
use super::sessionstore::SessionStore;

/// The sessioning middleware.
///
/// `Sessions` middleware is given a key-generating function and a
/// data store to use for sessioning.
///
/// The key is used to select a session from the store.
/// No session is actually created during selection. It is up to downstream
/// middleware to create/swap/edit sessions stored to a key.
///
/// `Sessions` allows guest sessioning (sessions without explicit authorization).
/// To prevent guest sessioning, the key generator can produce
/// an `Option` value so that all unauthorized users have an empty session.
///
/// Session keys can be stored in the `Request` or `Alloy`.
/// Usually, keys are stored in signed cookies, but anything
/// retrievable from `Request` or `Alloy` will work.
pub struct Sessions<K, V, S> {
    key_generator: fn(&Request, &Alloy) -> K,
    session_store: S
}

impl<K, V, S: SessionStore<K, V> + Clone> Clone for Sessions<K, V, S> {
    fn clone(&self) -> Sessions<K, V, S> {
        Sessions {
            key_generator: self.key_generator,
            session_store: self.session_store.clone()
        }
    }
}

impl<K, V, S: SessionStore<K, V>> Sessions<K, V, S> {
    /// Instantiate new sessioning middleware with the given
    /// key-generating function and session store.
    ///
    /// `key_generator` should generate keys based on the `Request` and `Alloy`.
    /// These keys should be unique, as identical keys will map to the same session.
    ///
    /// The `Alloy` can be used to access
    /// stores such as cookies to allow persistent sessions for users.
    ///
    /// `session_store` must implement the `SessionStore` trait.
    /// A default `Session` is provided to fulfill this.
    pub fn new(key_generator: fn(&Request, &Alloy) -> K,
               store: S) -> Sessions<K, V, S> {
        Sessions {
            key_generator: key_generator,
            session_store: store
        }
    }
}

impl<K, V, S: SessionStore<K, V> + Clone> Middleware for Sessions<K, V, S> {
    /// Adds the session store to the `alloy`.
    fn enter(&mut self, req: &mut Request, _: &mut Response,
             alloy: &mut Alloy) -> Status {
        // Generate and store the key for this session.
        let mut session = self.session_store.clone();
        session.select_session((self.key_generator)(req, alloy));

        // Add _all_ session store to the alloy.
        //     Anything added to the alloy must fulfill 'static,
        //     so we can't get to _this_ session under a ReadLockGuard.
        alloy.insert(session);

        Continue
    }
}
