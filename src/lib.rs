#![crate_id = "session"]
// #![deny(missing_doc)]
#![feature(phase)]
#![feature(globs)]

//! Session-storage middleware for the [Iron](https://ironframework.io/) web framework.
//!
//! The `sessions` module is used to create new sessioning middleware.
//!
//! `sessionstore` provides a default implementation of a session store.

extern crate collections;
extern crate core;
extern crate iron;
extern crate http;

pub use sessions::Sessions;
pub use sessionstore::SessionStore;
pub use sessionstore::session::Session;
// pub use sessionstore::hashsession::HashSession;

pub mod sessions;
pub mod sessionstore;
