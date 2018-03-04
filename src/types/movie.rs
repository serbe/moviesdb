use postgres::Connection;
use postgres::rows::Row;
use chrono::NaiveDateTime;

use db::Query;
use error::Error;
use types::Torrent;

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub id: i64,
    pub section: Option<String>,
    pub name: Option<String>,
    pub eng_name: Option<String>,
    pub year: Option<i32>,
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
    pub kinopoisk: Option<f32>,
    pub imdb: Option<f32>,
    pub poster: Option<String>,
    pub poster_url: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub torrents: Option<Vec<Torrent>>,
    pub nnm: Option<f32>,
}

impl Movie {
    pub fn new(
        id: i64,
        section: Option<String>,
        name: Option<String>,
        eng_name: Option<String>,
        year: Option<i32>,
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
        kinopoisk: Option<f32>,
        imdb: Option<f32>,
        poster: Option<String>,
        poster_url: Option<String>,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
        torrents: Option<Vec<Torrent>>,
        nnm: Option<f32>,
    ) -> Movie {
        Movie {
            id,
            section,
            name,
            eng_name,
            year,
            genre,
            country,
            raw_country,
            director,
            producer,
            actor,
            description,
            age,
            release_date,
            russian_date,
            duration,
            kinopoisk,
            imdb,
            poster,
            poster_url,
            created_at,
            updated_at,
            torrents,
            nnm,
        }
    }

    fn from_row(row: &Row) -> Movie {
        Movie::new(
            row.get("id"),
            row.get("section"),
            row.get("name"),
            row.get("eng_name"),
            row.get("year"),
            row.get("genre"),
            row.get("country"),
            row.get("raw_country"),
            row.get("director"),
            row.get("producer"),
            row.get("actor"),
            row.get("description"),
            row.get("age"),
            row.get("release_date"),
            row.get("russian_date"),
            row.get("duration"),
            row.get("kinopoisk"),
            row.get("imdb"),
            row.get("poster"),
            row.get("poster_url"),
            row.get("created_at"),
            row.get("updated_at"),
            None,
            None,
        )
    }
}

impl Query for Movie {
    fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
        let mut vec = Vec::new();
            for row in &conn.query("SELECT * FROM movies", &[])? {
            vec.push(Movie::from_row(&row));
        }
        Ok(vec)
    }

    fn get_by_id(conn: &Connection, id: i64) -> Result<Self, Error> {
        let rows = &conn.query("SELECT * FROM movies WHERE id = $1", &[&id])?;
        Ok(Movie::from_row(&rows.get(0)))
    }
}

// pub fn get_movies(conn: &Connection, page: i64) -> Result<Vec<Movie>, String> {
//     let limit: i64 = 100;
//     let offset: i64 = (page-1)*limit;
//     let mut movies: Vec<Movie> = Vec::new();
//     let query_str: &str = "
//         SELECT
//             *
//         FROM
//             movies
//         WHERE
//             id IN (
//                 SELECT
//                     t.movie_id
//                 FROM
//                     torrents AS t
//                 GROUP BY movie_id
//                 ORDER BY max(id) desc
//                 LIMIT $1
//                 OFFSET $2
//             )
//     ;";
//     for row in &conn.query(query_str, &[&limit, &offset]).unwrap() {
//         let mut movie: Movie = Movie::new();
//         movie.id = row.get("id");
//         movie.section = row.get("section");
//         movie.name = row.get("name");
//         movie.eng_name = row.get("eng_name");
//         movie.genre = row.get("genre");
//         movie.country = row.get("country");
//         movie.raw_country = row.get("raw_country");
//         movie.director = row.get("director");
//         movie.producer = row.get("producer");
//         movie.actor = row.get("actor");
//         movie.description = row.get("description");
//         movie.age = row.get("age");
//         movie.release_date = row.get("release_date");
//         movie.russian_date = row.get("russian_date");
//         movie.duration = row.get("duration");
//         movie.kinopoisk = row.get("kinopoisk");
//         movie.imdb = row.get("imdb");
//         movie.poster = row.get("poster");
//         movie.poster_url = row.get("poster_url");
//         movie.created_at = row.get("created_at");
//         movie.updated_at = row.get("updated_at");
//         movie.torrents = get_torrents(conn, movie.id);
//         movie.nnm = mean_nnm(&movie.torrents);
//         movies.push(movie);
//     }
//     if !movies.is_empty() {
//         Ok(movies)
//     } else {
//         Err("test".to_string())
//     }
// }

// pub fn mean_nnm(v: &Option<Vec<Torrent>>) -> Option<f32> {
//     match *v {
//         None => None,
//         Some(ref val) => {
//             let n: Vec<f32> = val.iter().map(|i| {
//                 i.nnm.unwrap_or(0.0)
//             }).filter(|i| {
//                 *i != 0.0
//             }).collect::<Vec<f32>>();
//             let k: f32 = n.iter().sum();
//             if k == 0. || n.is_empty() {
//                 None
//             } else {
//                 Some(k / n.len() as f32)
//             }
//         }
//     }
// }
