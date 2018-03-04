use postgres::Connection;
use postgres::rows::Row;
use chrono::NaiveDateTime;

use db::Query;
use error::Error;

#[derive(Debug, Serialize, Deserialize)]
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

impl Torrent {
    pub fn new(
        id: i64,
        movie_id: i64,
        date_create: Option<String>,
        href: Option<String>,
        torrent: Option<String>,
        magnet: Option<String>,
        nnm: Option<f32>,
        subtitles_type: Option<String>,
        subtitles: Option<String>,
        video: Option<String>,
        quality: Option<String>,
        resolution: Option<String>,
        audio1: Option<String>,
        audio2: Option<String>,
        audio3: Option<String>,
        translation: Option<String>,
        size: Option<i32>,
        seeders: Option<i32>,
        leechers: Option<i32>,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
    ) -> Torrent {
        Torrent {
            id,
            movie_id,
            date_create,
            href,
            torrent,
            magnet,
            nnm,
            subtitles_type,
            subtitles,
            video,
            quality,
            resolution,
            audio1,
            audio2,
            audio3,
            translation,
            size,
            seeders,
            leechers,
            created_at,
            updated_at,
        }
    }

    fn from_row(row: &Row) -> Torrent {
        Torrent::new(
            row.get("id"),
            row.get("movie_id"),
            row.get("date_create"),
            row.get("href"),
            row.get("torrent"),
            row.get("magnet"),
            row.get("nnm"),
            row.get("subtitles_type"),
            row.get("subtitles"),
            row.get("video"),
            row.get("quality"),
            row.get("resolution"),
            row.get("audio1"),
            row.get("audio2"),
            row.get("audio3"),
            row.get("translation"),
            row.get("size"),
            row.get("seeders"),
            row.get("leechers"),
            row.get("created_at"),
            row.get("updated_at"),
        )
    }
}

impl Query for Torrent {
    fn get_all(conn: &Connection) -> Result<Vec<Self>, Error> {
        let mut vec = Vec::new();
            for row in &conn.query("SELECT * FROM torrents", &[])? {
            vec.push(Torrent::from_row(&row));
        }
        Ok(vec)
    }

    fn get_by_id(conn: &Connection, id: i64) -> Result<Self, Error> {
        let rows = &conn.query("SELECT * FROM torrents WHERE id = $1", &[&id])?;
        Ok(Torrent::from_row(&rows.get(0)))
    }
}

// pub fn get_torrents(conn: &Connection, movie_id: i64) -> Option<Vec<Torrent>> {
//     let query_str: &str = "
//         SELECT
//             *
//         FROM
//             torrents
//         WHERE
//             movie_id = $1
//     ;";
//     let mut torrents = Vec::new();
//     for row in &conn.query(query_str, &[&movie_id]).unwrap() {
//         let mut torrent = Torrent::new();
//         torrent.id = row.get("id");
//         torrent.movie_id = row.get("movie_id");
//         torrent.date_create = row.get("date_create");
//         torrent.href = row.get("href");
//         torrent.torrent = row.get("torrent");
//         torrent.magnet = row.get("magnet");
//         torrent.nnm = row.get("nnm");
//         torrent.subtitles_type = row.get("subtitles_type");
//         torrent.subtitles = row.get("subtitles");
//         torrent.video = row.get("video");
//         torrent.quality = row.get("quality");
//         torrent.resolution = row.get("resolution");
//         torrent.audio1 = row.get("audio1");
//         torrent.audio2 = row.get("audio2");
//         torrent.audio3 = row.get("audio3");
//         torrent.translation = row.get("translation");
//         torrent.size = row.get("size");
//         torrent.seeders = row.get("seeders");
//         torrent.leechers = row.get("leechers");
//         torrent.created_at = row.get("created_at");
//         torrent.updated_at = row.get("updated_at");

//         torrents.push(torrent);
//     }
//     if !torrents.is_empty() {
//         Some(torrents)
//     } else {
//         None
//     }
// }
