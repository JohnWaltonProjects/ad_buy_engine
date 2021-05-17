use actix_web::web::{get, post, resource, Data, Json, Query};
use actix_web::{middleware, web, HttpResponse};
use actix_web::{App, HttpServer};
use ad_buy_engine::couch_rs;
use ad_buy_engine::couch_rs::types::document::DocumentCreatedDetails;
use ad_buy_engine::data::visit::{CouchedVisit, Visit};
use std::collections::HashMap;

pub type CouchClient = couch_rs::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("STARTED\n\n\n");

    let couch = ad_buy_engine::couch_rs::Client::new(
        "http://couch_database:5984",
        "couched_visits",
        "uX2b6@q5CxOjT7NrxYDc",
    )
    .expect("%GFDGSDHFG");

    HttpServer::new(move || {
        App::new()
            .data(couch.clone())
            .wrap(middleware::Logger::default())
            .service(resource("/make_db").route(get().to(make_db)))
            .service(resource("/insert_visit").route(post().to(insert_visit)))
            .service(resource("/upsert_visit").route(post().to(upsert_visit)))
            .service(resource("/restore_visit").route(get().to(restored_visit)))
            .default_service(web::route().to(get_health))
    })
    .bind("couch_app:9000")?
    .run()
    .await
}

pub async fn get_health() -> HttpResponse {
    println!("from couch_app responding to campaign_server request");
    HttpResponse::Ok().body("Healthy")
}

pub async fn upsert_visit(
    q: Query<HashMap<String, String>>,
    couch_client: Data<CouchClient>,
    payload: Json<Visit>,
) -> HttpResponse {
    let db_name = q.get("db_name").cloned().expect("db_name iddfs");
    let mut visit = CouchedVisit::from(payload.into_inner());

    match couch_client.db(&db_name).await {
        Ok(database) => match CouchedVisit::upsert(&mut visit, &database).await {
            Ok(res) => {
                return HttpResponse::Ok().finish();
            }

            Err(err) => {
                println!("Error 23: {:?}", err)
            }
        },

        Err(err) => {
            println!("failed to open db, not found?");
            println!("{}", err.message);
        }
    }
    HttpResponse::InternalServerError().finish()
}

pub async fn insert_visit(
    q: Query<HashMap<String, String>>,
    couch_client: Data<CouchClient>,
    payload: Json<Visit>,
) -> HttpResponse {
    println!("XXX");
    let db_name = q.get("db_name").cloned().expect("db_name iddfs");
    let mut visit = CouchedVisit::from(payload.into_inner());
    println!("XXX");
    match couch_client.db(&db_name).await {
        Ok(database) => {
            println!("345453");
            match CouchedVisit::insert(&mut visit, &database).await {
                Ok(res) => {
                    println!("doc created: {:?}", &res);
                    return HttpResponse::Ok().finish();
                }

                Err(err) => {
                    println!("Error 23: {:?}", err)
                }
            }
        }

        Err(err) => {
            println!("failed to open db, not found?");
            println!("{}", err.message);
        }
    }
    HttpResponse::InternalServerError().finish()
}

pub async fn restored_visit(
    q: Query<HashMap<String, String>>,
    couch_client: Data<CouchClient>,
) -> HttpResponse {
    let db_name = q.get("db_name").cloned().expect("db_name iddfs");
    let visit_id = q.get("visit_id").cloned().expect("visit_id g645");
    match couch_client.db(&db_name).await {
        Ok(database) => match CouchedVisit::get(&visit_id, &database).await {
            Ok(res) => {
                let payload: Visit = res.into();

                return HttpResponse::Ok().json(&payload);
            }

            Err(err) => {
                println!("Error 23: {:?}", err)
            }
        },

        Err(err) => {
            println!("failed to open db, not found?");
            println!("{}", err.message);
        }
    }
    HttpResponse::InternalServerError().finish()
}

pub async fn make_db(
    q: Query<HashMap<String, String>>,
    couch_client: Data<CouchClient>,
) -> HttpResponse {
    let db_name = q.get("db_name").cloned().expect("db_name iddfs");
    match couch_client.make_db(&db_name).await {
        Ok(ers) => {
            println!("db created");
        }
        Err(err) => {
            println!("failed db create");
            println!("{}", err.message);
        }
    }
    HttpResponse::Ok().finish()
}
