#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/session")]
#![deny(missing_doc)]
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
#[cfg(test)]
extern crate "iron-test" as test;
extern crate typemap;

pub use sessions::Sessions;
pub use sessionstore::SessionStore;
pub use sessionstore::session::Session;
pub use sessionstore::hashsession::HashSessionStore;

pub mod sessions;
pub mod sessionstore;
