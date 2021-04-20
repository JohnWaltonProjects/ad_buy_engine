use crate::api::campaign_state::find_campaign;
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use actix::Addr;
use actix_redis::RedisActor;
use actix_web::http::header::USER_AGENT;
use actix_web::web::{Data, Path, Query};
use actix_web::{HttpRequest, HttpResponse};
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::visit::geo_ip::GeoIPData;
use ad_buy_engine::data::visit::user_agent::UserAgentData;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

pub async fn process_click(
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
        let geo_ip = GeoIPData::new(ip);
        println!("{:?}", &geo_ip);
        // let user_agent = UserAgentData::new(ua.to_string());
        // println!("\nIP: {:?}\n", ip);

        // println!("{}", ua.to_str().unwrap());
        // let ua_data = UserAgentData::new()

        // build click map
        // build visit
        // visitor identity
        //  save to redis

        Ok(HttpResponse::Ok().finish())
    } else {
        Err(ApiError::NotFound(
            "no campaign found in appstate".to_string(),
        ))
    }
}
