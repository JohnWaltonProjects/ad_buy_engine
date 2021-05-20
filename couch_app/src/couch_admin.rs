use crate::typed_couch_docs::CouchUser;
use crate::CouchClient;
use crate::COUCH_CLIENT;
use actix_web::http::Method;
use ad_buy_engine::couch_rs::database::Database;
use ad_buy_engine::serde_json::{json, Value};
use tokio::time::Duration;

pub async fn create_database(
    client: CouchClient,
    name: &str,
) -> Result<(CouchClient, Database), String> {
    match client.make_db(name).await {
        Ok(res) => Ok((client, res)),
        Err(e) => Err(e.message),
    }
}

pub async fn create_user(
    client: CouchClient,
    username: &str,
    password: &str,
) -> Result<CouchClient, String> {
    let payload = CouchUser::new(username.to_string(), password.to_string());
    dbg!(ad_buy_engine::serde_json::to_string(&payload).expect("%$G#"));

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
    // let req = client
    //     .req(
    //         Method::DELETE,
    //         format!("/_users/org.couchdb.user:{}", username),
    //         None,
    //     )
    //     .send();
    // match req.await {
    //     Ok(res) => {
    //         if res.status().is_success() {
    //             println!("User Deleted");
    //             Ok(client)
    //         } else {
    //             match res.json::<Value>().await {
    //                 Ok(res) => Err(format!("User Not Created:{:?}", res)),
    //                 Err(err) => Err(format!("User Not Created:{:?}", err)),
    //             }
    //         }
    //     }
    //     Err(e) => Err(e.to_string()),
    // }
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

// #[tokio::test(flavor = "current_thread")]
// async fn create_test_database() {
//     std::thread::sleep(std::time::Duration::from_secs(1));
//     create_database(COUCH_CLIENT.clone(), "test").await.unwrap();
// }

// #[tokio::test(flavor = "current_thread")]
// async fn test_create_user() {
//     std::thread::sleep(std::time::Duration::from_secs(1));
//     create_user(COUCH_CLIENT.clone(), "test_user", "test_password")
//         .await
//         .unwrap();
// }

#[tokio::test(flavor = "current_thread")]
async fn test_delete_user() {
    std::thread::sleep(std::time::Duration::from_secs(1));
    delete_user(COUCH_CLIENT.clone(), "test_user")
        .await
        .unwrap();
}
//
// #[tokio::test(flavor = "current_thread")]
// async fn delete_test_database() {
//     std::thread::sleep(std::time::Duration::from_secs(1));
//     delete_database(COUCH_CLIENT.clone(), "test").await.unwrap();
// }
