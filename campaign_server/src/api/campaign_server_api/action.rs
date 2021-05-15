use crate::api::campaign_state::find_campaign;
use crate::utils::authentication::{decode_jwt, PrivateClaim};
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use crate::utils::helpers::respond_ok;
use actix::Addr;
use actix_identity::Identity;
use actix_redis::{Command, RedisActor, RespValue};
use actix_web::http::header::{LOCATION, REFERER, USER_AGENT};
use actix_web::web::{block, Data, Path, Query};
use actix_web::{HttpRequest, HttpResponse};
use ad_buy_engine::chrono::Local;
use ad_buy_engine::data::backend_models::linked_conversion::LinkedConversion;
use ad_buy_engine::data::backend_models::DatabaseCommunication;
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::elements::matrix::MatrixData;
use ad_buy_engine::data::visit::click_map::ClickMap;
use ad_buy_engine::data::visit::conversion::Conversion;
use ad_buy_engine::data::visit::geo_ip::GeoIPData;
use ad_buy_engine::data::visit::user_agent::UserAgentData;
use ad_buy_engine::data::visit::visit_identity::ClickIdentity;
use ad_buy_engine::data::visit::{CouchedVisit, Visit};
use ad_buy_engine::uuid::Uuid;
use ad_buy_engine::Url;
use std::collections::HashMap;
use std::net::IpAddr;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Mutex;

pub async fn action(
    req: HttpRequest,
    pool: Data<PgPool>,
    params: Query<HashMap<String, String>>,
    couch_rs: Data<ad_buy_engine::couch_rs::Client>,
) -> Result<HttpResponse, ApiError> {
    let sid = params.get("sid").cloned();

    if let Some(subid) = sid {
        let local_pool = pool.clone();
        let restored_linked_conversion =
            block(move || LinkedConversion::get(subid, local_pool.get().expect("T$G%H^%").deref()))
                .await?;

        let visit_id = &restored_linked_conversion.visit_id;
        let database_name = &restored_linked_conversion.account_id;
        let database = couch_rs.db(database_name).await?;
        let mut restored_visit = CouchedVisit::get(visit_id, &database).await?;

        let conversion = Conversion {
            postback_url_parameters: params.into_inner(),
            offer_id: Uuid::parse_str(&restored_linked_conversion.offer_id.clone()).expect("^H%Y"),
            postback_timestamp: Local::now().naive_local(),
        };
        restored_visit.conversions.push(conversion);

        CouchedVisit::update(&mut restored_visit, &database).await?;
    }

    respond_ok()
}
