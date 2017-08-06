extern crate postgres;

use postgres::{Connection};
use types::{Torrent, Movie};

// use types;

pub fn get_movies(conn: Connection, page: i64) -> Result<Vec<Movie>, String> {
    let limit: i64 = 100;
    let offset = (page-1)*limit;
    for row in &conn.query("
            SELECT
                *
            FROM
                movies
            WHERE
                id IN (
                    SELECT
                        t.movie_id
                    FROM
                        torrents AS t
                    GROUP BY movie_id
                    ORDER BY max(id) desc
                    LIMIT $1
                    OFFSET $2
                )
            ;", &[&limit, &offset]).unwrap() {
        // println!("{:?}", row);
        let mut movie = Movie::new();
        movie.id = row.get("id");
        movie.name = row.get("name");
        movie.imdb = row.get("imdb");
        println!("Movie {} {:?} {:?}", movie.id, movie.name, movie.imdb);
    }
    Err("test".to_string())
}

// pub fn get_torrents(movie_id: i64) -> Option<Vec<Torrent>> {

// }