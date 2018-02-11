use hyper::Error as HyperError;
use serde_json::Error as SerdeError;
use postgres::Error as PostgresError;

#[derive(Debug)]
pub enum Error {
  NotFound,
  Hyper(HyperError),
  Serde(SerdeError),
  Postgres(PostgresError)
}

impl Error {
  // pub fn from_serde(err: SerdeError) -> Error {
  //   Error::Serde(err)
  // }

  // return the error message in json format
  pub fn json(self) -> String {
    match self {
      Error::NotFound => r#"{"error":{"code":"error/not-found"}}"#.to_string(),
      Error::Hyper(_) => r#"{"error":{"code":"error/hyper"}}"#.to_string(),
      Error::Serde(_) => r#"{"error":{"code":"error/serde-json"}}"#.to_string(),
      Error::Postgres(_) => r#"{"error":{"code":"error/postgres-database"}}"#.to_string(),
    }
  }
}

macro_rules! impl_from {
  ($v:path, $t:ty) => {
    impl From<$t> for Error {
      fn from(err: $t) -> Self {
        $v(err)
      }
    }
  }
}

impl_from!(Error::Hyper, HyperError);
impl_from!(Error::Serde, SerdeError);
impl_from!(Error::Postgres, PostgresError);