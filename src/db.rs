extern crate postgres;

use postgres::{Connection};
use types::{Torrent, Movie};

// use types;

pub fn get_movies(conn: &Connection, page: i64) -> Result<Vec<Movie>, String> {
    let limit: i64 = 100;
    let offset = (page-1)*limit;
    let mut movies = Vec::new();
    let query_str = "
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
            ;";
    for row in &conn.query(query_str, &[&limit, &offset]).unwrap() {
        let mut movie = Movie::new();
        movie.id = row.get("id");
        movie.section = row.get("section");
        movie.name = row.get("name");
        movie.eng_name = row.get("eng_name");
        movie.genre = row.get("genre");
        movie.country = row.get("country");
        movie.raw_country = row.get("raw_country");
        movie.director = row.get("director");
        movie.producer = row.get("producer");
        movie.actor = row.get("actor");
        movie.description = row.get("description");
        movie.age = row.get("age");
        movie.release_date = row.get("release_date");
        movie.russian_date = row.get("russian_date");
        movie.duration = row.get("duration");
        movie.kinopoisk = row.get("kinopoisk");
        movie.imdb = row.get("imdb");
        movie.poster = row.get("poster");
        movie.poster_url = row.get("poster_url");
        // movie.created_at = row.get("created_at");
        // movie.updated_at = row.get("updated_at");
        movie.torrents = get_torrents(conn, movie.id);
        // movie.nnm = match movie.torrents {
        //     None => None,
        //     Some(ref val) => val.
        // };
        movies.push(movie);
    }
    if movies.len() > 0 {
        Ok(movies)
    } else {
        Err("test".to_string())
    }
}

pub fn get_torrents(conn: &Connection, movie_id: i64) -> Option<Vec<Torrent>> {
    let query_str = "
        SELECT
            *
        FROM
            torrents
        WHERE
            movie_id = $1
        ;";
    let mut torrents = Vec::new();
    for row in &conn.query(query_str, &[&movie_id]).unwrap() {
        let mut torrent = Torrent::new();
        torrent.id = row.get("id");
        torrent.movie_id = row.get("movie_id");
        torrent.date_create = row.get("date_create");
        torrent.href = row.get("href");
        torrent.torrent = row.get("torrent");
        torrent.magnet = row.get("magnet");
        torrent.nnm = row.get("nnm");
        torrent.subtitles_type = row.get("subtitles_type");
        torrent.subtitles = row.get("subtitles");
        torrent.video = row.get("video");
        torrent.quality = row.get("quality");
        torrent.resolution = row.get("resolution");
        torrent.audio1 = row.get("audio1");
        torrent.audio2 = row.get("audio2");
        torrent.audio3 = row.get("audio3");
        torrent.translation = row.get("translation");
        torrent.size = row.get("size");
        torrent.seeders = row.get("seeders");
        torrent.leechers = row.get("leechers");
        // torrent.created_at = row.get("created_at");
        // torrent.updated_at = row.get("updated_at");

        torrents.push(torrent);
    }
    if torrents.len() > 0 {
        Some(torrents)
    } else {
        None
    }
}