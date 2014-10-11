extern crate http;
extern crate iron;
extern crate session;

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use iron::{Iron, Request, Response, IronResult, Error};
use session::{SessionStore, Session};
use session::sessions::RequestSession;

// Echo the sessioned count to the client
fn get_count(req: &mut Request) -> IronResult<Response> {
    // Retrieve our session from the store
    let session = req.extensions.find_mut::<RequestSession, Session<SocketAddr, u32>>().unwrap();
    // Store or increase the sessioned count
    let count = session.upsert(1u32, |v: &mut u32| { *v = *v + 1; } );

    println!("{} hits from\t{}", count, req.remote_addr.unwrap())

    Ok(Response::with(::http::status::Ok, format!("Sessioned count: {}", count).as_slice()))
}


fn main() {
    Iron::new(get_count).listen(Ipv4Addr(127, 0, 0, 1), 3000);
}

// fn id_from_socket_addr(req: &Request) -> SocketAddr {
//     req.remote_addr.unwrap()
// }
