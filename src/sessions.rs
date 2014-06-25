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

impl<K: 'static, V, S: SessionStore<K, V> + Clone> Middleware for Sessions<K, V, S> {
    /// Adds the session store to the `alloy`.
    fn enter(&mut self, req: &mut Request, _: &mut Response,
             alloy: &mut Alloy) -> Status {
        // Retrieve the session for this request
        let session = self.session_store.select_session((self.key_generator)(req, alloy));

        // Store this session in the alloy
        alloy.insert(session);
        Continue
    }
}

#[cfg(test)]
mod test {
    pub use super::*;
    pub use super::super::sessionstore::*;
    pub use super::super::sessionstore::session::*;
    pub use super::super::sessionstore::hashsession::*;
    pub use iron::*;
    pub use iron::middleware::*;
    pub use std::sync::{Arc, Mutex};
    pub use std::mem::uninitialized;

    pub fn get_session_id(_: &Request, _: &Alloy) -> char {'a'}

    pub fn check_session_char_char(_: &mut Request, _: &mut Response, alloy: &mut Alloy) {
        let _ = alloy.find::<Session<char, char>>().unwrap();
    }
    pub fn check_session_char_u32(_: &mut Request, _: &mut Response, alloy: &mut Alloy) {
        let _ = alloy.find::<Session<char, u32>>().unwrap();
    }

    mod enter {
        use super::*;

        #[test]
        fn handles_multiple_sessions() {
            let mut test_server: ServerT = Iron::new();
            test_server.chain.link(Sessions::new(get_session_id, HashSessionStore::<char, char>::new()));
            test_server.chain.link(Sessions::new(get_session_id, HashSessionStore::<char, u32>::new()));
            test_server.chain.link(check_session_char_char);
            test_server.chain.link(check_session_char_u32);
            unsafe {
                let _ = test_server.chain.dispatch(
                    uninitialized(),
                    uninitialized(),
                    None
                );
            }
        }
    }
}
