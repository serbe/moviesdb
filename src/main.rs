//#![cfg_attr(feature="clippy", feature(plugin))]
//
//#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate chrono;
extern crate hyper;
extern crate futures;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;

mod db;
mod service;
mod types;
mod error;

use std::rc::Rc;
use hyper::server::Http;

use service::Api;
use db::Database;

fn main() {
    // let conn = db_conn();

    // let movies = get_movies(&conn, 1);

//    let sdb: Arc<Mutex<Connection>> = Arc::new(Mutex::new(conn));
    // let mut router = router::Router::new();

    // Iron::new(|_: &mut Request| {
    //     Ok(Response::with((status::Ok, "Hello World!")))
    // }).http("localhost:3000").unwrap();

    // println!("{:?}", movies.unwrap().len());
    let addr = "0.0.0.0:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || {
        let db = Rc::new(Database::new());
        Ok(Api::new(db))
    }).unwrap();
    server.run().unwrap();
}
