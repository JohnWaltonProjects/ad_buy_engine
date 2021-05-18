use crate::api::account::{get_account_model, get_all_accounts, update_account};
use crate::api::crud_element::process_crud;
use crate::api::health::get_team_id;
use crate::api::{
    auth::{login, logout},
    crud_element,
    health::get_health,
    invitation,
    user::create_user,
};
use crate::management::api::debug;
use crate::management::api::email::get_email_list;
use crate::utils::middleware::auth::Auth as AuthMiddleware;
use actix_files::Files;
use actix_web::web::{get, post, resource, service, Data};
use actix_web::{web, HttpResponse};
use ad_buy_engine::constant::apis::private::{
    API_CRUD_ELEMENT, API_GET_ACCOUNT, API_POST_ACCOUNT, API_URL_LOGOUT,
};
use ad_buy_engine::constant::apis::public::{
    API_URL_CONFIRM_EMAIL_INVITATION, API_URL_CREATE_INVITATION, API_URL_CREATE_REGISTER,
    API_URL_LOGIN,
};
use ad_buy_engine::constant::local_system_location::{
    DIRECTORY_LOCATION_MAIN_PUBLIC_STATIC, DIRECTORY_LOCATION_MAIN_PUBLIC_TERTIARY_STATIC,
    DIRECTORY_LOCATION_MAIN_SECURE_STATIC,
};
use ad_buy_engine::string_manipulation::backend::api_path_builder::{
    parse_api_v2_url, parse_v1_api, trim_api_v1,
};

pub fn public_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(resource(API_URL_LOGIN).route(post().to(login)))
        .service(
            web::scope("/api/v2").service(
                web::scope("/invitation")
                    .service(resource("/new_invitation").route(post().to(invitation::create)))
                    .service(resource("/register").route(post().to(create_user)))
                    .service(
                        resource("/confirm_email_invitation/{id}")
                            .route(get().to(invitation::update)),
                    ),
            ),
        )
        .service(
            web::scope("/tertiary").default_service(
                Files::new("", "./static/main/public/tertiary")
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
        );
}
