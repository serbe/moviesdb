extern crate chrono;

use self::chrono::NaiveDateTime;

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
