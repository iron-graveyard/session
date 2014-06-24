#![crate_id = "session"]
// #![deny(missing_doc)]
#![feature(phase)]

//! Session-storage middleware for the [Iron](https://www.github.com/iron/iron) web framework.

extern crate collections;
extern crate core;
extern crate iron;
extern crate http;

pub use sessions::Sessions;
pub use sessionstore::SessionStore;
pub use sessionstore::store::Session;

pub mod sessions;
pub mod sessionstore;
