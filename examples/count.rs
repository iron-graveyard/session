 #![allow(unused_must_use)]
 
extern crate iron;
extern crate session;

use session::{Sessions, SessionStore, HashSessionStore};
use session::sessions::RequestSession;

use iron::{Request, Response, IronResult, Chain, Iron};
use iron::typemap;

fn handle_request(req: &mut Request) -> IronResult<Response> {
    if req.url.path[0] == "favicon.ico" {
       Ok(Response::with((iron::status::Ok))) 
    } else {
        // Retrieve our session from the store
        let session = req.extensions.get_mut::<RequestSession<MySessionKey>>();

        let mut count = 0;
        match session {
            None => {
                println!("{}", "session has not been set yet!")
            },
            Some(v) => {
                match v.find() {
                    None => {
                        count = v.upsert(1u32, count_func)
                    },
                    Some(v2) => {
                        // Store or increase the sessioned count
                        count = v.upsert(v2, count_func)
                    }
                }     
            },
        }

        println!("{} hits from\t{}", count, req.remote_addr);

        Ok(Response::with((iron::status::Ok, format!("Sessioned count: {:?}", count))))
    }
}

fn count_func(v: &mut u32) {  
    *v = *v + 1 
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct MySessionKey(u32);
impl typemap::Key for MySessionKey { type Value = u32; }

fn id_generator(_: &Request) -> MySessionKey { 
    MySessionKey(1u32)
}

fn main() {
    let mut chain = Chain::new(handle_request);

    let hs: HashSessionStore<MySessionKey> = HashSessionStore::new();
    let s: Sessions<MySessionKey, HashSessionStore<MySessionKey>> = Sessions::new(id_generator, hs);
    chain.link_before(s);
    let server = Iron::new(chain);
    server.http("localhost:3000");
}