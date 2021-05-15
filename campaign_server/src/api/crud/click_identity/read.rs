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

pub async fn get_identity(
    redis: &Addr<RedisActor>,
    ua: &str,
    ip: IpAddr,
) -> Result<ClickIdentity, ApiError> {
    let req = redis.send(Command(resp_array![
        "GET",
        format!("{}:{}", &ua, ip).as_str(),
    ]));

    match req.await {
        Ok(res) => match res {
            Ok(rv) => {
                if let RespValue::SimpleString(data) = rv {
                    Ok(ad_buy_engine::serde_json::from_str(&data).expect("G%Rf"))
                } else {
                    Err(ApiError::BadRequest(
                        "Resp val not simple string".to_string(),
                    ))
                }
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
//     cm: ClickMap,
//     redis: &Addr<RedisActor>,
//     pool: Data<PgPool>,
//     click_identity: ClickIdentity,
//     visit: Visit,
// ) -> Result<Url, ApiError> {
//     let ci_modal = click_identity.clone().into();
//     let res =
//         create_click_identity(&click_identity.user_agent, click_identity.ip, cm, redis).await?;
//     let res = block(move || click_identity::create_click_identity(&pool, ci_modal)).await?;
//     Ok(())
// }
//
// pub async fn create_click_identity(
//     ua: &str,
//     ip: IpAddr,
//     cm: ClickMap,
//     redis: &Addr<RedisActor>,
// ) -> Result<(), ApiError> {
//     let req = redis.send(Command(resp_array![
//         "SET",
//         format!("{}{}", ua, ip).as_str(),
//         serde_json::to_string(&cm).expect("TYgsdfg")
//     ]));
//     match req.await {
//         Ok(res) => match res {
//             Ok(rv) => {
//                 println!("{:?}", &rv);
//                 Ok(())
//             }
//             Err(e) => {
//                 println!("{:?}", e);
//                 Err(ApiError::InternalServerError(format!("{:?}", e)))
//             }
//         },
//         Err(e) => {
//             println!("{:?}", e);
//             Err(ApiError::InternalServerError(format!("redis err")))
//         }
//     }
// }
//
// pub async fn get_click_identity(
//     ua: &str,
//     ip: IpAddr,
//     redis: Data<Addr<RedisActor>>,
// ) -> Result<ClickMap, ApiError> {
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
