use crate::couch_admin::{
    add_security_document_to_database, couchdb_create_user_database, create_database, create_user,
};
use crate::{CouchClient, COUCH_CLIENT, COUCH_SERVER_URI};
use actix_web::web::{Data, Json, Query};
use actix_web::HttpResponse;
use ad_buy_engine::data::visit::{CouchedVisit, Visit};
use std::collections::HashMap;

pub async fn new_user(q: Query<HashMap<String, String>>) -> HttpResponse {
    let username = q.get("username").cloned().expect("username");
    let password = q.get("password").cloned().expect("password");
    let database_name = q.get("database_name").cloned().expect("database_name");
    match couchdb_create_user_database(username, password, database_name).await {
        Ok(res) => {
            println!("User Created");
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            println!("User Not Created");
            HttpResponse::BadRequest().json(&err)
        }
    }
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

pub async fn make_db(q: Query<HashMap<String, String>>) -> HttpResponse {
    let res = create_user(COUCH_CLIENT.clone(), "test_user_x", "test_password_x")
        .await
        .unwrap();
    let res = create_database(res, "test_user_x").await.unwrap();
    let res = add_security_document_to_database(
        res.0,
        "test_user_x".to_string(),
        "test_user_x".to_string(),
    )
    .await
    .unwrap();
    let user_client =
        ad_buy_engine::couch_rs::Client::new(COUCH_SERVER_URI, "test_user_x", "test_password_x")
            .unwrap();
    let res = user_client.db("test_user_x").await.unwrap();
    println!("Success: {:?}", res);
    HttpResponse::Ok().json("Ok")
}
