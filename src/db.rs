use postgres::{Connection, TlsMode};

use error::Error;
use types::*;

pub struct Database {
  conn: Connection
}

impl Database {
    pub fn new() -> Database {
        Database{
            conn: Connection::connect("postgres://movuser:movpass@localhost:5432/movies", TlsMode::None)
                .expect("Error connection to database.")
        }
    }

    pub fn get_all<T: Query + 'static>(&self) -> Result<String, Error> {
        T::get_all(&self.conn)
            .and_then(|ref v| to_string(v).map_err(|e| Error::Serde(e)))
    }
}

pub trait Query where Self: Sized + Serialize {
//   fn create(conn: &Connection, obj: Self) -> Result<(), Error>;
//   fn get_by_id(conn: &Connection, id: i32) -> Result<Self, Error>;
  fn get_all(conn: &Connection) -> Result<Vec<Self>, Error>;
//   fn update(conn: &Connection, obj: Self) -> Result<(), Error>;
//   fn delete_by_id(conn: &Connection, id: i32) -> Result<(), Error>;
}
