use crate::api::account::get_all_accounts;
use crate::api::health::get_health;
use crate::management::api::debug::{delete_all_funnels, reset_users_accounts_emls};
use crate::management::api::email::get_email_list;
use crate::server::{create_doc, restored_visit, test_make, upsert_doc};
use actix_web::web::{get, resource, ServiceConfig};
use actix_web::HttpResponse;
use ad_buy_engine::string_manipulation::backend::api_path_builder::{
    parse_api_v2_url, parse_v1_api, trim_api_v1,
};

pub fn test_routes(cfg: &mut ServiceConfig) {
    cfg.route("/health", get().to(get_health))
        .service(resource("/test_m").route(get().to(test_make)))
        .service(resource("/test_c").route(get().to(create_doc)))
        .service(resource("/test_r").route(get().to(restored_visit)))
        .service(resource("/test_u").route(get().to(upsert_doc)))
        .service(resource("/version").to(|| async { HttpResponse::Ok().body("Version 1.2") }))
        .service(resource("/reset_user_account_eml").route(get().to(reset_users_accounts_emls)))
        .service(resource("/get_all_accounts").route(get().to(get_all_accounts)))
        .service(resource("/delete_all_funnels").route(get().to(delete_all_funnels)))
        .service(resource("/get_all_emails").route(get().to(get_email_list)));
}
