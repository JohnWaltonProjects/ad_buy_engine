use crate::typed_couch_docs::{CouchSecurity, CouchUser};
use crate::COUCH_CLIENT;
use crate::{CouchClient, COUCH_SERVER_URI};
use actix_web::http::Method;
use ad_buy_engine::couch_rs::database::Database;
use ad_buy_engine::serde_json::{json, Value};
use tokio::time::Duration;

pub async fn create_database(
    couch_admin_client: CouchClient,
    name: &str,
) -> Result<(CouchClient, Database), String> {
    match couch_admin_client.make_db(name).await {
        Ok(res) => Ok((couch_admin_client, res)),
        Err(e) => Err(e.message),
    }
}

pub async fn create_user(
    client: CouchClient,
    username: &str,
    password: &str,
) -> Result<CouchClient, String> {
    let payload = CouchUser::new(username.to_string(), password.to_string());
    // dbg!(ad_buy_engine::serde_json::to_string(&payload).expect("%$G#"));

    let req = client
        .req(
            Method::PUT,
            format!("/_users/org.couchdb.user:{}", username),
            None,
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send();

    match req.await {
        Ok(res) => {
            if res.status().is_success() {
                println!("User Created");
                Ok(client)
            } else {
                println!("User Not Created");
                match res.json::<Value>().await {
                    Ok(res) => Err(format!("User Not Created:{:?}", res)),
                    Err(err) => Err(format!("User Not Created:{:?}", err)),
                }
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn delete_user(client: CouchClient, username: &str) -> Result<CouchClient, String> {
    match client.db("_users").await {
        Ok(res) => {
            match res
                .get::<Value>(&format!("org.couchdb.user:{}", username))
                .await
            {
                Ok(val) => {
                    println!("User Got: {:?}", &val);
                    match res.remove(&val).await {
                        true => Ok(client),
                        false => Err(format!("User Not Deleted")),
                    }
                }
                Err(e) => Err(e.message),
            }
        }
        Err(e) => Err(e.message),
    }
}

pub async fn delete_database(client: CouchClient, name: &str) -> Result<CouchClient, String> {
    match client.destroy_db(name).await {
        Ok(res) => Ok((client)),
        Err(e) => Err(e.message),
    }
}

pub async fn add_security_document_to_database(
    admin_couch_client: CouchClient,
    database_name: String,
    username: String,
) -> Result<(), String> {
    let security_document = CouchSecurity::for_user(username);
    let req = admin_couch_client
        .req(Method::PUT, format!("/{}/_security", database_name), None)
        .header("Accept", "application/json")
        .header("Accept", "text/plain")
        .json(&security_document)
        .send();
    match req.await {
        Ok(res) => match res.status().is_success() {
            true => {
                dbg!("Success: {:?}", res);
                Ok(())
            }
            false => {
                dbg!("Response returned failed status");
                Err(format!("{:?}", res))
            }
        },
        Err(e) => {
            dbg!(&e);
            Err(e.to_string())
        }
    }
}

pub async fn couchdb_create_user_database(
    username: String,
    password: String,
    database_name: String,
) -> Result<(), String> {
    let res = create_user(COUCH_CLIENT.clone(), &username, &password).await?;
    let res = create_database(res, &database_name).await?;
    Ok(add_security_document_to_database(res.0, database_name, username).await?)
}
