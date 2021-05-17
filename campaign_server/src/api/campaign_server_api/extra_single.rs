use crate::api::campaign_server_api::{find_depth, from_request_extract_identity};
use crate::api::campaign_state::find_campaign;
use crate::api::crud::click_identity::write::store_initial_click;
use crate::helper_functions::http_request_functions::extract_matrix_id;
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use actix::Addr;
use actix_redis::{Command, RedisActor, RespValue};
use actix_web::http::header::{LOCATION, REFERER, USER_AGENT};
use actix_web::web::{block, Data, Path, Query};
use actix_web::{HttpRequest, HttpResponse};
use ad_buy_engine::data::backend_models::linked_conversion::LinkedConversion;
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::data::elements::funnel::SequenceType;
use ad_buy_engine::data::elements::matrix::MatrixData;
use ad_buy_engine::data::visit::click_event::{ClickEvent, ClickableElement, TerseElement};
use ad_buy_engine::data::visit::click_map::ClickMap;
use ad_buy_engine::data::visit::geo_ip::GeoIPData;
use ad_buy_engine::data::visit::user_agent::UserAgentData;
use ad_buy_engine::data::visit::visit_identity::ClickIdentity;
use ad_buy_engine::data::visit::Visit;
use ad_buy_engine::traversal::Bft;
use ad_buy_engine::Url;
use ad_buy_engine::Uuid;
use std::collections::HashMap;
use std::net::IpAddr;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Mutex;

pub async fn extra_single(
    req: HttpRequest,
    pool: Data<PgPool>,
    app_state: Data<Mutex<HashMap<Uuid, Campaign>>>,
    redis: Data<Addr<RedisActor>>,
) -> Result<HttpResponse, ApiError> {
    let restored_click_identity = from_request_extract_identity(&req, &redis, &pool).await?;
    let restored_click_map = restored_click_identity.click_map.clone();
    let visit_id = restored_click_identity.visit_id.clone();
    let account_id = restored_click_identity.account_id.clone();
    let lean_account_id = account_id.chars().filter(|s| *s != '-').collect::<String>();

    let mut restored_visit: Visit = reqwest::Client::default()
        .get(&format!(
            "http://couch_app:9000/restore_visit?db_name={}&visit_id={}",
            &lean_account_id, &visit_id
        ))
        .send()
        .await?
        .json::<Visit>()
        .await?;

    let campaign_id = restored_visit.campaign_id.clone();

    if let Some(sequence_type) = restored_click_identity.click_map.seq_type {
        match sequence_type {
            SequenceType::Matrix => {
                let matrix_id = extract_matrix_id(&req)?;
                let found_node = restored_click_map.find_node_in_matrix(matrix_id);

                let selected_node = found_node.children.first().unwrap().value.clone();
                let mut url = new_string!("");

                match selected_node.data {
                    MatrixData::Offer(offer) => {
                        url = offer.url.to_string();
                        restored_visit.push_click_event(ClickEvent::create(
                            ClickableElement::Offer(TerseElement::new(
                                selected_node.id.clone(),
                                None,
                            )),
                        ));

                        let local_pool = pool.clone();

                        let account_id = account_id.clone();
                        let visit_id = visit_id.clone();
                        let campaign_id = campaign_id.to_string();

                        let block_result = block(move || {
                            LinkedConversion::new(
                                LinkedConversion::create(
                                    &account_id,
                                    &visit_id,
                                    &campaign_id,
                                    &offer.offer_id,
                                ),
                                local_pool.get().expect("THRDF").deref(),
                            )
                        })
                        .await?;
                    }

                    MatrixData::LandingPage(lp) => {
                        url = lp.url.to_string();
                        restored_visit.push_click_event(ClickEvent::create(
                            ClickableElement::LandingPage(TerseElement::new(
                                selected_node.id.clone(),
                                Some(lp.url.clone()),
                            )),
                        ));
                    }
                    _ => {}
                }

                reqwest::Client::default()
                    .post(&format!(
                        "http://couch_app:9000/upsert_visit?db_name={}",
                        &lean_account_id,
                    ))
                    .header("Content-Type", "application/json")
                    .json(&restored_visit)
                    .send()
                    .await?;

                Ok(HttpResponse::Found().header(LOCATION, url).finish())
            }

            _ => {
                let offer_click_map = restored_click_identity
                    .click_map
                    .children
                    .first()
                    .expect("G%$tfsdg")
                    .clone();

                if let MatrixData::Offer(offer) = offer_click_map.value.data {
                    let redirect_url = offer.url.clone();

                    restored_visit.push_click_event(ClickEvent::create(ClickableElement::Offer(
                        TerseElement::new(offer.offer_id, Some(offer.url.clone())),
                    )));

                    reqwest::Client::default()
                        .post(&format!(
                            "http://couch_app:9000/upsert_visit?db_name={}",
                            &lean_account_id
                        ))
                        .header("Content-Type", "application/json")
                        .json(&restored_visit)
                        .send()
                        .await?;

                    let local_pool = pool.clone();
                    let account_id = account_id.clone();
                    let campaign_id = campaign_id.to_string();
                    let visit_id = visit_id.clone();

                    let result = block(move || {
                        LinkedConversion::new(
                            LinkedConversion::create(
                                &account_id,
                                &visit_id,
                                &campaign_id,
                                &offer.offer_id,
                            ),
                            local_pool.get().expect("Y^%JH").deref(),
                        )
                    })
                    .await?;

                    Ok(HttpResponse::Found()
                        .header(LOCATION, redirect_url.as_str())
                        .finish())
                } else {
                    Err(ApiError::InternalServerError("not an offer".to_string()))
                }
            }
        }
    } else {
        Err(ApiError::InternalServerError(
            "no sequence found".to_string(),
        ))
    }
}
