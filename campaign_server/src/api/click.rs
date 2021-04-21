use crate::api::campaign_state::find_campaign;
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use actix::Addr;
use actix_redis::{RedisActor, Command, RespValue};
use actix_web::http::header::{LOCATION, REFERER, USER_AGENT};
use actix_web::web::{Data, Path, Query};
use actix_web::{HttpRequest, HttpResponse};
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::visit::geo_ip::GeoIPData;
// #[]
use ad_buy_engine::data::elements::matrix::MatrixData;
use ad_buy_engine::data::visit::click_map::ClickMap;
use ad_buy_engine::data::visit::user_agent::UserAgentData;
use ad_buy_engine::Url;
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Mutex;
use uuid::Uuid;
use ad_buy_engine::data::visit::Visit;

pub async fn process_click(
    req: HttpRequest,
    pool: Data<PgPool>,
    app_state: Data<Mutex<HashMap<Uuid, Campaign>>>,
    redis: Data<Addr<RedisActor>>,
    campaign_id: Path<Uuid>,
    traffic_source_parameters: Query<HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    // try if visit identity exists then modify that visit record
    // if existing visit ident and new campaign id, new visit?
    redis.send(Command)
    if let Some(found) = find_campaign(campaign_id.into_inner(), app_state, &pool) {
        let ua = req.headers().get(USER_AGENT).unwrap().to_str().unwrap();
        let ip = req.peer_addr().unwrap().ip();
        let ip = IpAddr::from_str("172.58.38.197").unwrap();

        let geo_ip = GeoIPData::new(ip);
        let user_agent = UserAgentData::new(ua.to_string());
        let referrer = if let Some(x) = req.headers().get(REFERER) {
            if let Ok(z) = x.to_str() {
                if let Ok(y) = Url::parse(z) {
                    println!("\n\n\n{}", &y);
                    Some(y)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let click_map = ClickMap::from_campaign(
            &found,
            &geo_ip,
            &user_agent,
            &traffic_source_parameters,
            referrer,
        );
        
        let mut visit = Visit::new()


        // build click map
        // build visit
        // visitor identity
        //  save to redis
        // send to agent to save identity to pg
        /// setup redis recache on server start up

        Ok(HttpResponse::Ok().finish())
    } else {
        Err(ApiError::NotFound(
            "no campaign found in appstate".to_string(),
        ))
    }
}


pub async fn insert_click_identity(ua: &str, ip:IpAddr,cm:ClickMap, redis:Data<Addr<RedisActor>>) -> Result<(), ApiError> {
let req=    redis.send(Command(resp_array!["SET", format!("{}{}", ua,ip).as_str(), serde_json::to_string(&cm).expect("TYgsdfg")]));
    match req.await {
        Ok(res)=>{
            match res {
                Ok(rv)=>{
                    println!("{:?}",&rv);
                    Ok(())
                }
                Err(e)=>{
                    println!("{:?}",e);
                    Err(ApiError::InternalServerError(format!("{:?}",e)))
                }
            }
        }
        Err(e)=>{
            println!("{:?}",e);
            Err(ApiError::InternalServerError(format!("redis err")))
        }
    }
}

pub async fn get_click_identity(ua: &str, ip:IpAddr, redis:Data<Addr<RedisActor>>) -> Result<ClickMap, ApiError> {
    let req=    redis.send(Command(resp_array!["GET", format!("{}{}", ua,ip).as_str(), serde_json::to_string(&cm).expect("TYgsdfg")]));
    match req.await {
        Ok(res)=>{
            match res {
                Ok(rv)=>{
                    println!("{:?}",&rv);
                    
                    if let RespValue::SimpleString(str)=rv  {
                        if let Ok(cm) = serde_json::from_str(&str) {
                            Ok(cm)
                        } else {
                            Err(ApiError::InternalServerError(format!("json serde err")))
                        }
                    } else {
                        Err(ApiError::InternalServerError(format!("")))
                    }
                    
                    Ok(rcv)
                }
                Err(e)=>{
                    println!("{:?}",e);
                    Err(ApiError::BadRequest(format!("{:?}",e)))
                }
            }
        }
        Err(e)=>{
            println!("{:?}",e);
            Err(ApiError::InternalServerError(format!("redis err")))
        }
    }
}

// find ident from database else insert it and add to cache

pub async fn find_identity_from_database(){

}