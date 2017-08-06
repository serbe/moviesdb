extern crate postgres;

use postgres::{Connection, TlsMode};
use db::{get_movies};

mod types;
mod db;

fn main() {
    let conn = Connection::connect("postgres://movuser:movpass@localhost:5432/movies", TlsMode::None).unwrap();
    // println!("{:?}", conn);

    let movies = get_movies(conn, 1);

    println!("{:?}", movies);

    // for row in &conn.query("select * from torrents", &[]).unwrap() {
    //     let mut torrent = types::Torrent::new();
    //     torrent.id = row.get("id");
    //     torrent.movie_id = row.get("movie_id");
    //     torrent.resolution = row.get("resolution");
    //     println!("Torrent {} {:?} {:?}", torrent.id, torrent.movie_id, torrent.resolution);
    // }
}