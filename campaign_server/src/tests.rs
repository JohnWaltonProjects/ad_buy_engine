use crate::api::account::get_all_accounts;
use crate::api::health::get_health;
use crate::management::api::debug::{delete_all_funnels, reset_users_accounts_emls};
use crate::management::api::email::get_email_list;
use crate::management::couch::COUCH_APP_URI;
use crate::server::{create_doc, restored_visit, upsert_doc};
use crate::utils::errors::ApiError;
use actix_web::web::{get, resource, ServiceConfig};
use actix_web::HttpResponse;
use ad_buy_engine::string_manipulation::backend::api_path_builder::{
    parse_api_v2_url, parse_v1_api, trim_api_v1,
};

#[actix_rt::test]
pub async fn main_test() {
    // let res = reqwest::Client::default()
    //     .get("http://127.0.0.1:9000/new_user?username=testa&password=testa&database_name=testa")
    //     .send()
    //     .await
    //     .unwrap();
    let url = format!(
        "{}new_user?username=testusername&password=testadsfsdf&database_name=testb",
        COUCH_APP_URI,
    );
    dbg!(&url);

    dbg!(reqwest::Client::default()
        .get(&url)
        .send()
        .await
        .map_err(|e| ApiError::InternalServerError(format!("make db err: {:?}", e)))
        .unwrap());
    // let res = reqwest::Client::default().get(COUCH_APP_URI).send().await;
    // res.unwrap();
}

// #[actix_rt::test]
// pub async fn test_create_couch_database() {
//     let url = format!(
//         "{}new_user?username={}&password={}&database_name={}",
//         COUCH_APP_URI, "test_username", "test_password", "test_database",
//     );
//
//     let res = reqwest::Client::default().get(&url).send().await;
//     match res {
//         Ok(res) => {
//             if !res.status().is_success() {
//                 panic!("Error: {:?}", res)
//             }
//         }
//         Err(err) => {
//             panic!("err")
//         }
//     }
// }
