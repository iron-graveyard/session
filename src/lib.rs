#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/session")]

//! Session-storage middleware for the [Iron](https://ironframework.io/) web framework.
//!
//! The `sessions` module is used to create new sessioning middleware.
//!
//! `sessionstore` provides a default implementation of a session store.

#![feature(collections)]
#![feature(reflect_marker)]

extern crate collections;
extern crate core;
extern crate iron;
#[cfg(test)]
extern crate iron_test as test;

pub use sessions::Sessions;
pub use sessionstore::SessionStore;
pub use sessionstore::session::Session;
pub use sessionstore::hashsession::HashSessionStore;

pub mod sessions;
pub mod sessionstore;
