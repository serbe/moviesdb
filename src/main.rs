extern crate postgres;
extern crate time;

use postgres::{Connection, TlsMode};
use time::Timespec;


struct Movie {
    id: i64,
    section: Option<String>,
    name: Option<String>,
    eng_name: Option<String>,
    year: Option<i64>,
    genre: Option<Vec<String>>,
    country: Option<Vec<String>>,
    raw_country: Option<String>,
    director: Option<Vec<String>>,
    producer: Option<Vec<String>>,
    actor: Option<Vec<String>>,
    description: Option<String>,
    age: Option<String>,
    release_date: Option<String>,
    russian_date: Option<String>,
    duration: Option<String>,
    kinopoisk: Option<f64>,
    imdb: Option<f64>,
    poster: Option<String>,
    poster_url: Option<String>,
    created_at: Option<Timespec>,
    updated_at: Option<Timespec>,
}

struct Torrent {
  id: i64,
  movie_id: i64,
  date_create: Option<String>,
  href: Option<String>,
  torrent: Option<String>,
  magnet: Option<String>,
  nnm: Option<f64>,
  subtitles_type: Option<String>,
  subtitles: Option<String>,
  video: Option<String>,
  quality: Option<String>,
  resolution: Option<String>,
  audio1: Option<String>,
  audio2: Option<String>,
  audio3: Option<String>,
  translation: Option<String>,
  size: Option<i64>,
  seeders: Option<i64>,
  leechers: Option<i64>,
  created_at: Option<Timespec>,
  updated_at: Option<Timespec>,
}

impl Movie {
    pub fn new() -> Self {
        Movie {
            id: 0,
            section: None,
            name: None,
            eng_name: None,
            year: None,
            genre: None,
            country: None,
            raw_country: None,
            director: None,
            producer: None,
            actor: None,
            description: None,
            age: None,
            release_date: None,
            russian_date: None,
            duration: None,
            kinopoisk: None,
            imdb: None,
            poster: None,
            poster_url: None,
            created_at: None,
            updated_at: None,
        }
    }
}

impl Torrent {
    pub fn new() -> Self {
        Torrent {
            id: 0,
            movie_id: 0,
            date_create: None,
            href: None,
            torrent: None,
            magnet: None,
            nnm: None,
            subtitles_type: None,
            subtitles: None,
            video: None,
            quality: None,
            resolution: None,
            audio1: None,
            audio2: None,
            audio3: None,
            translation: None,
            size: None,
            seeders: None,
            leechers: None,
            created_at: None,
            updated_at: None,
        }
    }
}

fn main() {
    // let conn = Connection::connect("postgres://movuser:movpass@localhost:5432/movies", TlsMode::None).unwrap();
    // for row in &conn.query("select * from torrents", &[]).unwrap() {
    //     let mut torrent = Torrent::new();
    //     torrent.id = row.get("id");
    //     torrent.movie_id = row.get("movie_id");
    //     torrent.resolution = row.get("resolution");
    //     println!("Torrent {} {:?} {:?}", torrent.id, torrent.movie_id, torrent.resolution);
    // }
}