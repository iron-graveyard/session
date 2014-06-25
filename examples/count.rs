extern crate http;
extern crate iron;
extern crate session;

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use iron::{Iron, ServerT, Chain, Request, Response, Alloy};
use iron::mixin::Serve;
use session::{Sessions, SessionStore, HashSessionStore, Session};

// Echo the sessioned count to the client
fn get_count(req: &mut Request, res: &mut Response, alloy: &mut Alloy) {
    // Retrieve our session from the store
    let session = alloy.find_mut::<Session<SocketAddr, u32>>().unwrap();
    // Store or increase the sessioned count
    let count = session.upsert(1u32, |v: &mut u32| { *v = *v + 1; } );

    println!("{} hits from\t{}", count, req.remote_addr.unwrap())
    let _ = res.serve(::http::status::Ok, format!("Sessioned count: {}", count).as_slice());
}

fn main() {
    let mut server: ServerT = Iron::new();
    server.chain.link(Sessions::new(id_from_socket_addr, HashSessionStore::<SocketAddr, u32>::new()));
    server.chain.link(get_count);
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}

fn id_from_socket_addr(req: &Request, _: &Alloy) -> SocketAddr {
    req.remote_addr.unwrap()
}
