extern crate postgres;
extern crate iron;

use iron::prelude::*;
use iron::status;
use db::{get_movies, db_conn};

mod models;
mod db;

fn main() {
    let conn = match db_conn() {
        Ok(c) => c,
        Err(_) => return
    };

    let movies = get_movies(&conn, 1);

    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World!")))
    }).http("localhost:3000").unwrap();

    println!("{:?}", movies.unwrap().len());
}