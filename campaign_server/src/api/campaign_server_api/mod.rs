use crate::api::crud::click_identity::read;
use crate::api::crud::click_identity::write;
use crate::db::crud::click_identity;
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use actix::Addr;
use actix_redis::RedisActor;
use actix_web::http::header::USER_AGENT;
use actix_web::web::{block, Data};
use actix_web::HttpRequest;
use ad_buy_engine::data::visit::visit_identity::ClickIdentity;
use std::collections::HashMap;

pub mod action;
pub mod click;
pub mod extra_multiple;
pub mod extra_single;

pub fn find_depth(params: &HashMap<String, String>) -> Option<usize> {
    if let Some(x) = params.get("d") {
        if let Ok(y) = x.parse::<usize>() {
            Some(y)
        } else {
            None
        }
    } else {
        None
    }
}

pub async fn from_request_extract_identity(
    req: &HttpRequest,
    redis: &Data<Addr<RedisActor>>,
    pool: &Data<PgPool>,
) -> Result<ClickIdentity, ApiError> {
    let pool = pool.clone();
    let ua = req.headers().get(USER_AGENT).unwrap().to_str().unwrap();
    let ip = req.peer_addr().unwrap().ip();
    let ua_ip_id = format!("{}:{}", &ua, ip);

    let res = read::get_identity(redis, ua, ip).await;
    match res {
        Ok(d) => Ok(d),
        Err(e) => {
            // let pool = &pool.get()?;
            let res = block(move || click_identity::get_click_identity(&pool, ua_ip_id)).await?;
            let click_identity: ClickIdentity = res.into();
            write::create_click_identity(&click_identity, redis).await?;
            Ok(click_identity)
        }
    }
}
