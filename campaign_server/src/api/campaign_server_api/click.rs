use crate::api::campaign_state::find_campaign;
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use actix::Addr;
use actix_redis::{Command, RedisActor, RespValue};
use actix_web::http::header::{LOCATION, REFERER, USER_AGENT};
use actix_web::web::{Data, Path, Query};
use actix_web::{HttpRequest, HttpResponse};
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::visit::geo_ip::GeoIPData;
// #[]
use crate::api::crud::click_identity::write::store_initial_click;
use ad_buy_engine::data::elements::matrix::MatrixData;
use ad_buy_engine::data::visit::click_map::ClickMap;
use ad_buy_engine::data::visit::user_agent::UserAgentData;
use ad_buy_engine::data::visit::visit_identity::ClickIdentity;
use ad_buy_engine::data::visit::Visit;
use ad_buy_engine::{Url, generate_random_string};
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Mutex;
use uuid::Uuid;
use ad_buy_engine::data::backend_models::linked_conversion::LinkedConversion;

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
        ///dev
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

        let mut click_map = ClickMap::from_campaign(
            &found,
            &geo_ip,
            &user_agent,
            &traffic_source_parameters,
            referrer.clone(),
        );

        if let  MatrixData::Offer(offer)=&click_map.value.data {
            // create
            click_map./// create linked conversion for offer, and then save that modal
        }
        
        let (init_url, click_event) = click_map.get_initial_click()?;
        let visit = Visit::new(
            &found,
            geo_ip,
            user_agent,
            referrer,
            traffic_source_parameters.into_inner(),
            click_map.clone(),
            click_event,
        );
        let click_identity = ClickIdentity::new(visit.id, ua.to_string(), ip, cm);
        store_initial_click(redis.into_inner().as_ref(), pool, &click_identity, visit).await?;
        
        let new =LinkedConversion {
            id:generate_random_string(24),
            offer_id:Some()
        };
        //store linked conversion
        generate_random_string
        // generate random 24 digit string for offer
        /// need to link offer coonversion to the code
        Ok(HttpResponse::Found().header(LOCATION, init_url).finish())
    } else {
        Err(ApiError::NotFound(
            "no campaign found in appstate".to_string(),
        ))
    }
}
