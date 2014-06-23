extern crate http;
extern crate iron;
extern crate session;

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use iron::{Iron, ServerT, Request, Response, Alloy};
use iron::mixin::Serve;
use session::{Session, SessionStore};

// Echo the sessioned count to the client
fn get_count(req: &mut Request, res: &mut Response, alloy: &mut Alloy) {
    // A non-trivial use case may use cookies to identify users, see:
    //   https://github.com/iron/cookie
    // For the sake of example, sessions will be keyed by socket address
    let id = match req.remote_addr {
        Some(socket_addr) => socket_addr,
        None              => from_str("127.0.0.1").unwrap()
    };

    // The session store can be accessed under a RWLock
    let mut session_store = alloy.find_mut::<SessionStore<SocketAddr, u32>>().unwrap().write();
    // Map the sessions by socket address and store or increase the sessioned count
    // session_store is generic: any consistent key, value pair can be stored
    let count = session_store.insert_or_update_with(id, 1u32, |_k, v| { *v = *v + 1; } );

    println!("{} hits from\t{}", count, id)
    let _ = res.serve(::http::status::Ok, format!("Sessioned count: {}", count).as_slice());
}

fn main() {
    let mut server: ServerT = Iron::new();
    server.link(Session::<SocketAddr, u32>::new());
    server.link(get_count);
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
