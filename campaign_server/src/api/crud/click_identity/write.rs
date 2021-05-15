use crate::db::crud::click_identity;
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use actix::Addr;
use actix_redis::{Command, RedisActor, RespValue};
use actix_web::web::{block, Data};
use ad_buy_engine::data::backend_models::click_identity::ClickIdentityModal;
use ad_buy_engine::data::visit::click_map::ClickMap;
use ad_buy_engine::data::visit::visit_identity::ClickIdentity;
use ad_buy_engine::data::visit::Visit;
use ad_buy_engine::Url;
use std::net::IpAddr;

pub async fn store_initial_click(
    redis: &Addr<RedisActor>,
    pool: Data<PgPool>,
    click_identity: ClickIdentity,
    visit: Visit,
) -> Result<Url, ApiError> {
    let ci_modal = click_identity.clone().into();
    let res = create_click_identity(click_identity, redis).await?;
    let res = block(move || click_identity::create_click_identity(&pool, ci_modal)).await?;
    Err(ApiError::InternalServerError(format!("testing ")))
}

pub async fn create_click_identity(
    payload: ClickIdentity,
    redis: &Addr<RedisActor>,
) -> Result<(), ApiError> {
    let req = redis.send(Command(resp_array![
        "SET",
        payload.ua_ip_id.clone(),
        ad_buy_engine::serde_json::to_string(&payload).expect("TYgsdfg")
    ]));
    match req.await {
        Ok(res) => match res {
            Ok(rv) => {
                println!("{:?}", &rv);
                Ok(())
            }
            Err(e) => {
                println!("{:?}", e);
                Err(ApiError::InternalServerError(format!("{:?}", e)))
            }
        },
        Err(e) => {
            println!("{:?}", e);
            Err(ApiError::InternalServerError(format!("redis err")))
        }
    }
}

// pub async fn get_click_identity(
//     ua: &str,
//     ip: IpAddr,
//     redis: Data<Addr<RedisActor>>,
// ) -> Result<ClickMapStorage, ApiError> {
//     let req = redis.send(Command(resp_array![
//         "GET",
//         format!("{}{}", ua, ip).as_str()
//     ]));
//     match req.await {
//         Ok(res) => match res {
//             Ok(rv) => {
//                 println!("{:?}", &rv);
//
//                 if let RespValue::SimpleString(str) = rv {
//                     if let Ok(cm) = serde_json::from_str(&str) {
//                         Ok(cm)
//                     } else {
//                         Err(ApiError::InternalServerError(format!("json serde err")))
//                     }
//                 } else {
//                     Err(ApiError::InternalServerError(format!(
//                         "Resp val not string"
//                     )))
//                 }
//
//                 Ok(rcv)
//             }
//             Err(e) => {
//                 println!("{:?}", e);
//                 Err(ApiError::BadRequest(format!("{:?}", e)))
//             }
//         },
//         Err(e) => {
//             println!("{:?}", e);
//             Err(ApiError::InternalServerError(format!("redis err")))
//         }
//     }
// }
