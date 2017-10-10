#![cfg_attr(feature="clippy", feature(plugin))]

#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate postgres;
// extern crate iron;

// use iron::prelude::*;
// use iron::status;

//use std::sync::{Arc, Mutex};

use db::{get_movies, db_conn};

mod models;
mod db;

fn main() {
    let conn = db_conn();

    let movies = get_movies(&conn, 1);

//    let sdb: Arc<Mutex<Connection>> = Arc::new(Mutex::new(conn));
    // let mut router = router::Router::new();

    // Iron::new(|_: &mut Request| {
    //     Ok(Response::with((status::Ok, "Hello World!")))
    // }).http("localhost:3000").unwrap();

    println!("{:?}", movies.unwrap().len());
}