//! Sessioning middleware
//!
//! Instantiate and link a new `Sessions` struct to
//! give your server sessioning functionality.
//!
//! Key-generating functions and custom stores can be used
//! to customize functionality.

use iron::{ Request, BeforeMiddleware, IronResult, typemap };
use sessionstore::session;
use super::sessionstore::SessionStore;
use std::any::Any;
use std::boxed::Box;
use std::marker::PhantomData;
use std::marker::Reflect;

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
pub struct Sessions<K, S> {
    key_generator: fn(&Request) -> K,
    session_store: S
}

impl<K, S: SessionStore<K, Box<Any>> + Clone> Clone for Sessions<K, S> {
    fn clone(&self) -> Sessions<K, S> {
        Sessions {
            key_generator: self.key_generator,
            session_store: self.session_store.clone()
        }
    }
}

impl<K, S: SessionStore<K, Box<Any>>> Sessions<K, S> {
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
    pub fn new(key_generator: fn(&Request) -> K,
               store: S) -> Sessions<K, S> {
        Sessions {
            key_generator: key_generator,
            session_store: store
        }
    }
}

/// Key for inserting a Session<K, V> in the request extensions.
pub struct RequestSession<K> {
    phantom: PhantomData<K>
}

//impl<K: 'static, V: 'static> Assoc<session::Session<K, V>> for RequestSession {}
impl<K: 'static + Reflect> typemap::Key for RequestSession<K> { type Value = session::Session<K>; }

impl<K: 'static + Reflect, S: SessionStore<K, Box<Any>> + 'static + Clone> BeforeMiddleware for Sessions<K, S> {
    /// Adds the session store to the `alloy`.
    fn before(&self, req: &mut Request) -> IronResult<()> {
        // Retrieve the session for this request
        let session = self.session_store.select_session((self.key_generator)(req));

        // Store this session in the alloy
        req.extensions.insert::<RequestSession<K>>(session);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    pub use super::*;
    pub use super::super::sessionstore::*;
    pub use super::super::sessionstore::session::*;
    pub use super::super::sessionstore::hashsession::*;
    pub use iron::*;
    pub use test::mock::{request, response};
    pub use std::sync::{Arc, Mutex};

    pub fn get_session_id(_: &Request) -> char {'a'}

    pub fn check_session_char_char(req: &mut Request) -> IronResult<()> {
        let _ = req.extensions.find::<RequestSession, Session<char, char>>().unwrap();
        Ok(())
    }
    pub fn check_session_char_u32(req: &mut Request) -> IronResult<()> {
        let _ = req.extensions.find::<RequestSession, Session<char, u32>>().unwrap();
        Ok(())
    }

    mod enter {
        use super::*;

        fn dummy(_: &mut Request) -> IronResult<Response> {
            Ok(Response::new())
        }

        #[test]
        fn handles_multiple_sessions() {
            let mut chain = ChainBuilder::new(dummy);
            chain.link_before(Sessions::new(get_session_id, HashSessionStore::<char, char>::new()));
            chain.link_before(Sessions::new(get_session_id, HashSessionStore::<char, u32>::new()));
            chain.link_before(check_session_char_char);
            chain.link_before(check_session_char_u32);
            let _ = Iron::new(chain).http("localhost:3000");
        }
    }
}
