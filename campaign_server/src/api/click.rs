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
use ad_buy_engine::data::visit::visit_identity::ClickIdentity;

pub async fn process_initial_click(
    req: HttpRequest,
    pool: Data<PgPool>,
    app_state: Data<Mutex<HashMap<Uuid, Campaign>>>,
    redis: Data<Addr<RedisActor>>,
    campaign_id: Path<Uuid>,
    traffic_source_parameters: Query<HashMap<String, String>>,
) -> Result<HttpResponse, ApiError> {
    
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
            referrer.clone(),
        );

        let  visit = Visit::new(&found, geo_ip, user_agent, referrer, traffic_source_parameters.into_inner(), click_map.clone());
        let click_identity = ClickIdentity::new(visit.id,ua.to_string(), ip, cm);
        crate::api::crud::click_identity::store::store_initial_click(click_map, redis.into_inner().as_ref(),pool, click_identity, visit)
        
        ///  save that data to visit before storing

        Ok(HttpResponse::Ok().finish())
    } else {
        Err(ApiError::NotFound(
            "no campaign found in appstate".to_string(),
        ))
    }
}
