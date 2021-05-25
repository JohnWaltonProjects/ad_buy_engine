use crate::api::user::CouchUserDetails;
use crate::utils::errors::ApiError;
use actix_web::HttpResponse;
use ad_buy_engine::data::visit::Visit;
use ad_buy_engine::Uuid;
use reqwest::{Response, Url};

pub const COUCH_APP_URI: &'static str = "http://couch_app:9000/";

pub async fn create_couch_database(user_details: CouchUserDetails) -> Result<(), ApiError> {
    let url = format!(
        "{}new_user?username={}&password={}&database_name={}",
        COUCH_APP_URI, user_details.username, user_details.password, user_details.database_name,
    );
    dbg!(&url);

    dbg!(reqwest::Client::default()
        .get(&url)
        .send()
        .await
        .map_err(|e| ApiError::InternalServerError(format!("make db err: {:?}", e)))?);

    Ok(())
}

pub async fn restore_visit(db_name: String, visit_id: String) -> Result<Visit, ApiError> {
    let url = format!(
        "http://couch_app:9000/restore_visit?db_name={}&visit_id={}",
        db_name, visit_id
    );
    let url = Url::parse(&url)
        .map_err(|e| ApiError::InternalServerError(format!("parse err: {:?}", e)))?;

    match reqwest::Client::default()
        .get(url)
        .send()
        .await
        .map_err(|e| ApiError::InternalServerError(format!("make db err: {:?}", e)))?
        .json::<Visit>()
        .await
    {
        Ok(result) => Ok(result),
        Err(err) => {
            println!("Error:{:?}", &err);
            Err(ApiError::InternalServerError(format!(
                "json parse err \n\n\n"
            )))
        }
    }
}

pub async fn insert_visit(db_name: String, visit: Visit) -> Result<(), ApiError> {
    let url = format!("http://couch_app:9000/insert_visit?db_name={}", db_name,);
    let url = Url::parse(&url)
        .map_err(|e| ApiError::InternalServerError(format!("parse err: {:?}", e)))?;

    if let Ok(res) = reqwest::Client::default()
        .post(url)
        .header("Content-Type", "application/json")
        .json(&visit)
        .send()
        .await
        .map_err(|e| ApiError::InternalServerError(format!("make db err: {:?}", e)))
    {
        if res.status().is_success() {
            Ok(())
        } else {
            Err(ApiError::ClientSendRequestError(format!("Err :G^XXXXXX4")))
        }
    } else {
        Err(ApiError::ClientSendRequestError(format!("Err :G^$T44")))
    }
}

pub async fn upsert(db_name: String, visit: Visit) -> Result<(), ApiError> {
    let url = format!("http://couch_app:9000/upsert_visit?db_name={}", db_name,);
    let url = Url::parse(&url)
        .map_err(|e| ApiError::InternalServerError(format!("parse err: {:?}", e)))?;

    if let Ok(res) = reqwest::Client::default()
        .post(url)
        .header("Content-Type", "application/json")
        .json(&visit)
        .send()
        .await
        .map_err(|e| ApiError::InternalServerError(format!("make db err: {:?}", e)))
    {
        Ok(())
    } else {
        Err(ApiError::ClientSendRequestError(format!("Err :G^$T44")))
    }
}
