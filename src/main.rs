extern crate postgres;

use db::{get_movies, db_conn};

mod models;
mod db;

fn main() {
    let conn = match db_conn() {
        Ok(c) => c,
        Err(_) => return
    };

    let movies = get_movies(&conn, 1);

    println!("{:?}", movies.unwrap().len());
}