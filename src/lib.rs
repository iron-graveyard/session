#![crate_id = "session"]
#![deny(missing_doc)]
#![feature(phase)]

//! Session-storage middleware for the [Iron](https://www.github.com/iron/iron) web framework.

extern crate collections;
extern crate core;
extern crate iron;
extern crate http;

pub use session::{SessionStore, Session};

pub mod session;
