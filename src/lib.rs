#![crate_id = "session"]
#![deny(missing_doc)]
#![feature(phase)]

//! Session-storage middleware for the [Iron](https://ironframework.io/) web framework.
//!
//! New sessioning middleware can be made with the `sessions` module.
//!
//! A default implementation of a session store is provided in the `sessionstore` module.
//!
//! All module structs and types can be access through the crate, using type synonmyms.

extern crate collections;
extern crate core;
extern crate iron;
extern crate http;

pub use sessions::Sessions;
pub use sessionstore::SessionStore;
pub use sessionstore::store::Session;

pub mod sessions;
pub mod sessionstore;
