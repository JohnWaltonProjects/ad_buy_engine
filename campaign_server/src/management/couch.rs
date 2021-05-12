use crate::utils::errors::ApiError;
use actix_web::web::Data;
use ad_buy_engine::data::account::Account;
use couch_rs::document::DocumentCollection;
use couch_rs::types::find::FindQuery;
use serde_json::Value;
use std::error::Error;

pub async fn create_couch_database(
    account: &Account,
    couch_client: Data<couch_rs::Client>,
) -> Result<(), ApiError> {
    let make_db_result = couch_client
        .make_db(account.account_id.to_string().as_str())
        .await?;
    Ok(())
}

pub async fn destroy_couch_database(
    account: &Account,
    couch_client: Data<couch_rs::Client>,
) -> Result<(), ApiError> {
    let make_db_result = couch_client
        .destroy_db(account.account_id.to_string().as_str())
        .await?;
    Ok(())
}
