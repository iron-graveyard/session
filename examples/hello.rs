extern crate iron;
extern crate session;

use std::io::net::ip::Ipv4Addr;

use iron::{Iron, ServerT};

use session::HelloWorld;

fn main() {
    let mut server: ServerT = Iron::new();
    server.smelt(HelloWorld::new());
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
