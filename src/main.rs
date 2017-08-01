extern crate postgres;

use postgres::{Connection, TlsMode};


fn main() {
    // let conn = Connection::connect("postgres://movuser:movpass@localhost:5432/movies", TlsMode::None).unwrap();
    // println!("{:?}", conn);
    // for row in &conn.query("select * from torrents", &[]).unwrap() {
    //     let mut torrent = Torrent::new();
    //     torrent.id = row.get("id");
    //     torrent.movie_id = row.get("movie_id");
    //     torrent.resolution = row.get("resolution");
    //     println!("Torrent {} {:?} {:?}", torrent.id, torrent.movie_id, torrent.resolution);
    // }
}