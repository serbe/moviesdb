extern crate hyper;
extern crate futures;

use hyper::{Get, StatusCode, Body, Headers};
use futures::{future, Future, Stream};
use hyper::header::ContentLength;
use std::rc::Rc;
use hyper::server::{Request, Response, Service};

use db::Database;
use error::Error;
use types::*;

const MOVIES: &str = "/api/movies";
const TORRENTS: &str = "/api/torrents";
// const OTHER: &'static str = "/api/test";
// const RECEIPT: &'static str = "/api/receipt";
// const CUSTOMER: &'static str = "/api/customer";
// const PURCHASE: &'static str = "/api/purchase";
// const LOGIN: &'static str = "/api/login";

pub struct Api {
  db: Rc<Database>
}

impl Api {
    pub fn new(db: Rc<Database>) -> Api {
        Api { db }
    }

    fn get_body(body: Body, headers: &Headers) -> Box<Future<Item=Vec<u8>, Error=Error>> {
        let vec;
        if let Some(len) = headers.get::<ContentLength>() {
            vec = Vec::with_capacity(**len as usize);
        } else {
            vec = vec![];
        }
        Box::new(
        body
            .fold(vec, |mut acc, chunk| {
                acc.extend_from_slice(&*chunk);
                Ok::<_, hyper::Error>(acc)
                })
                .map_err(|e| Error::Hyper(e))
        )
    }

    fn error_res(err: Error) -> Result<Response, hyper::Error> {
        // error!("{:?}", err);
        Ok(Response::new()
            .with_status(StatusCode::InternalServerError)
            .with_body(err.json()))
    }
}

impl Service for Api {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let _db = Rc::clone(&self.db);
        let (method, uri, _http_version, _headers, _body) = req.deconstruct();
        match method {
            Get => Box::new(
                future::done(match uri.path() {
                    MOVIES => self.db.get_all::<Movie>(),
                    TORRENTS => self.db.get_all::<Torrent>(),
                    // CUSTOMER => self.db.get_all::<Customer>(),
                    // PURCHASE => self.db.join_purchase(),
                    // RECEIPT => self.db.join_receipt(),
                    // STAFF => self.db.join_staff(),
                    _ => {
                        println!("{:?} {:?} {:?}", method, uri, uri.query());
                        Err(Error::NotFound)
                    }
                })
                    .and_then(|s| Ok(Response::new().with_body(s)))
                    .or_else(Api::error_res)
            ),
            // Delete => Box::new(Api::get_body(body, headers)
            //     .and_then(move |chunks| {
            //     from_slice::<Id>(&chunks).map(|res| res.id).map_err(Error::from_serde)
            //     })
            //     .and_then(move |id| match uri.path() {
            //     GOODS => db.delete_by_id::<SuppliedGoods>(id),
            //     CUSTOMER => db.delete_by_id::<Customer>(id),
            //     PURCHASE => db.delete_by_id::<Purchase>(id),
            //     RECEIPT => db.delete_by_id::<Receipt>(id),
            //     STAFF => db.delete_by_id::<Staff>(id),
            //     _ => Err(Error::NotFound)
            //     })
            //     .map(|_| Response::new().with_body("[]".to_string()))
            //     .or_else(Api::error_res)
            // ),
            // Post => Box::new(Api::get_body(body, headers)
            //     .and_then(move |chunks| match uri.path() {
            //     GOODS => from_slice::<SuppliedGoods>(&chunks).map_err(Error::from_serde)
            //         .and_then(move |obj| db.create::<SuppliedGoods>(obj)).map(|_| "[]".to_string()),
            //     CUSTOMER => from_slice::<Customer>(&chunks).map_err(Error::from_serde)
            //         .and_then(move |obj| db.create::<Customer>(obj)).map(|_| "[]".to_string()),
            //     PURCHASE => from_slice::<Purchase>(&chunks).map_err(Error::from_serde)
            //         .and_then(move |obj| db.create::<Purchase>(obj)).map(|_| "[]".to_string()),
            //     RECEIPT => from_slice::<Receipt>(&chunks).map_err(Error::from_serde)
            //         .and_then(move |obj| db.create::<Receipt>(obj)).map(|_| "[]".to_string()),
            //     STAFF => from_slice::<Staff>(&chunks).map_err(Error::from_serde)
            //         .and_then(move |obj| db.create::<Staff>(obj)).map(|_| "[]".to_string()),
            //     LOGIN => from_slice::<LoginRequest>(&chunks).map_err(Error::from_serde)
            //         .and_then(move |obj| db.login(obj)),
            //     _ => Err(Error::NotFound)
            //     })
            //     .map(|body| Response::new().with_body(body))
            //     .or_else(Api::error_res)
            // ),
            // Put => Box::new(Api::get_body(body, headers)
            //     .and_then(move |chunks| match uri.path() {
            //     GOODS => from_slice::<SuppliedGoods>(&chunks).map_err(Error::from_serde)
            //         .and_then(move |obj| db.update::<SuppliedGoods>(obj)),
            //     CUSTOMER => from_slice::<Customer>(&chunks).map_err(Error::from_serde)
            //         .and_then(move |obj| db.update::<Customer>(obj)),
            //     PURCHASE => from_slice::<Purchase>(&chunks).map_err(Error::from_serde)
            //         .and_then(move |obj| db.update::<Purchase>(obj)),
            //     RECEIPT => from_slice::<Receipt>(&chunks).map_err(Error::from_serde)
            //         .and_then(move |obj| db.update::<Receipt>(obj)),
            //     STAFF => from_slice::<Staff>(&chunks).map_err(Error::from_serde)
            //         .and_then(move |obj| db.update::<Staff>(obj)),
            //     _ => Err(Error::NotFound)
            //     })
            //     .map(|_| Response::new().with_body("[]".to_string()))
            //     .or_else(Api::error_res)
            // ),
            _ => Box::new(future::err(Error::NotFound).or_else(Api::error_res))
        }
    }
}
