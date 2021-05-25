#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
use crate::apis::new_user;
use crate::couch_admin::{
    add_security_document_to_database, couchdb_create_user_database, create_database, create_user,
};
use actix_web::web::{block, get, post, resource, Data, Json, Query};
use actix_web::{middleware, web, HttpResponse};
use actix_web::{App, HttpServer};
use ad_buy_engine::couch_rs;
use ad_buy_engine::couch_rs::types::document::DocumentCreatedDetails;
use ad_buy_engine::data::visit::{CouchedVisit, Visit};
use reqwest::{Method, Url};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub mod apis;
pub mod couch_admin;
pub mod test;
pub mod typed_couch_docs;

pub type CouchClient = couch_rs::Client;
pub const COUCH_SERVER_URI: &'static str = "http://host.docker.internal:5984";
lazy_static! {
    pub static ref COUCH_CLIENT: ad_buy_engine::couch_rs::Client = {
        ad_buy_engine::couch_rs::Client::new(COUCH_SERVER_URI, "admin", "uX2b6@q5CxOjT7NrxYDc")
            .expect("%GFDGSDHFG")
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("STARTED\n\n\n");

    let couch = COUCH_CLIENT.clone();

    HttpServer::new(move || {
        App::new()
            .data(couch.clone())
            .wrap(middleware::Logger::default())
            .service(resource("/new_user").route(get().to(new_user)))
            .service(resource("/test").route(get().to(test_ping)))
            // .service(resource("/health").route(get().to(get_health)))
            // .service(resource("/insert_visit").route(post().to(insert_visit)))
            // .service(resource("/upsert_visit").route(post().to(upsert_visit)))
            // .service(resource("/restore_visit").route(get().to(restored_visit)))
            .default_service(web::route().to(get_health))
    })
    .bind("couch_app:9000")?
    .run()
    .await
}

pub async fn get_health() -> HttpResponse {
    HttpResponse::Ok().body("Healthy")
}

pub async fn test_ping() -> HttpResponse {
    println!("Pinging couchdb on host os");

    dbg!(reqwest::Client::default()
        .get("http://host.docker.internal:5984")
        .send()
        .await
        .unwrap());

    HttpResponse::Ok().body("Healthy")
}
