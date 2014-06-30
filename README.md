session [![Build Status](https://secure.travis-ci.org/iron/session.png?branch=master)](https://travis-ci.org/iron/session)
====

> Sessioning middleware for the [Iron](https://github.com/iron/iron) web framework.

## Example

```rust
extern crate http;
extern crate iron;
extern crate session;

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use iron::{Iron, ServerT, Chain, Request, Response, Alloy};
use iron::middleware::{Status, Continue, FromFn};
use iron::mixin::Serve;
use session::{Sessions, SessionStore, HashSessionStore, Session};

// Echo the sessioned count to the client
fn get_count(req: &mut Request, res: &mut Response, alloy: &mut Alloy) -> Status {
    // Retrieve our session from the store
    let session = alloy.find_mut::<Session<SocketAddr, u32>>().unwrap();
    // Store or increase the sessioned count
    let count = session.upsert(1u32, |v: &mut u32| { *v = *v + 1; } );

    println!("{} hits from\t{}", count, req.remote_addr.unwrap())
    let _ = res.serve(::http::status::Ok, format!("Sessioned count: {}", count).as_slice());

    Continue
}

fn main() {
    let mut server: ServerT = Iron::new();
    server.chain.link(Sessions::new(id_from_socket_addr, HashSessionStore::<id_type, u32>::new()));
    server.chain.link(FromFn::new(get_count));
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}

fn id_generator(req: &Request, al: &Alloy) -> id_type { ... }
```

## Overview

session is a part of Iron's [core bundle](https://github.com/iron/core).

- Includes an implemented `HashMap`-based session store
- Key sessions based on your own id generating function
- Store and retrieve data to/from keyed sessions

## Installation

If you're using a `Cargo` to manage dependencies, just add session to the toml:

```toml
[dependencies.session]

git = "https://github.com/iron/session.git"
```

Otherwise, `cargo build`, and the rlib will be in your `target` directory.

## [Documentation](http://docs.ironframework.io/core/session)

Along with the [online documentation](http://docs.ironframework.io/core/session),
you can build a local copy with `make doc`.

## [Examples](/examples)

## Get Help

One of us ([@reem](https://github.com/reem/), [@zzmp](https://github.com/zzmp/),
[@theptrk](https://github.com/theptrk/), [@mcreinhard](https://github.com/mcreinhard))
is usually on `#iron` on the mozilla irc. Come say hi and ask any questions you might have.
We are also usually on `#rust` and `#rust-webdev`.
