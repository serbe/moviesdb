extern crate chrono;

use self::chrono::NaiveDateTime;

#[derive (Debug)]
pub struct Movie {
    pub id: i64,
    pub section: Option<String>,
    pub name: Option<String>,
    pub eng_name: Option<String>,
    pub year: Option<i64>,
    pub genre: Option<Vec<String>>,
    pub country: Option<Vec<String>>,
    pub raw_country: Option<String>,
    pub director: Option<Vec<String>>,
    pub producer: Option<Vec<String>>,
    pub actor: Option<Vec<String>>,
    pub description: Option<String>,
    pub age: Option<String>,
    pub release_date: Option<String>,
    pub russian_date: Option<String>,
    pub duration: Option<String>,
    pub kinopoisk: Option<f64>,
    pub imdb: Option<f64>,
    pub poster: Option<String>,
    pub poster_url: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub torrents: Option<Vec<Torrent>>,
	pub nnm: Option<f32>,
}

#[derive(Debug)]
pub struct Torrent {
  pub id: i64,
  pub movie_id: i64,
  pub date_create: Option<String>,
  pub href: Option<String>,
  pub torrent: Option<String>,
  pub magnet: Option<String>,
  pub nnm: Option<f32>,
  pub subtitles_type: Option<String>,
  pub subtitles: Option<String>,
  pub video: Option<String>,
  pub quality: Option<String>,
  pub resolution: Option<String>,
  pub audio1: Option<String>,
  pub audio2: Option<String>,
  pub audio3: Option<String>,
  pub translation: Option<String>,
  pub size: Option<i32>,
  pub seeders: Option<i32>,
  pub leechers: Option<i32>,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>,
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
            torrents: None,
            nnm: None,
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