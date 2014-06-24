use iron::{Request, Response, Middleware, Alloy};
use iron::middleware::{Status, Continue};
use super::sessionstore::SessionStore;

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
        // Generate and store the key for this session
        self.session_store.set_key((self.key_generator)(req, alloy));

        // Add _all_ session store to the alloy
        //     Anything added to the alloy must fulfill 'static,
        //     so we can't get to _this_ session under a ReadLockGuard
        alloy.insert(self.session_store.clone());

        Continue
    }
}
