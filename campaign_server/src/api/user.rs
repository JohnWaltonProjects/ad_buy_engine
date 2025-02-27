use actix_web::client::Client;
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;
use diesel::prelude::*;
use ad_buy_engine::{CreateUserRequest, UserResponse};
use ad_buy_engine::data::account::Account;
use ad_buy_engine::data::user::User;

use crate::dns::dns_cname::request_subdomain;
use crate::management;
use crate::management::api;
use crate::db;
use crate::db::user_depricating::*;
use crate::schema::accounts::dsl::{id as account_id, accounts};
use crate::schema::invitation::dsl::invitation;
use crate::schema::users::dsl::{id as user_id, users};
use crate::utils::authentication::hash;
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use crate::utils::helpers::{redirect_to, respond_json, respond_ok};

pub async fn get_user(
    uid: Path<Uuid>,
    pool: Data<PgPool>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = block(move || find(&pool, uid.clone())).await?;
    respond_json(user)
}

pub async fn create_user(
    client: Data<Client>,
    pool: Data<PgPool>,
    params: Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let pool_a = pool.clone();
    let pool_b = pool.clone();
    let pool_c = pool.clone();
    let params_a = params.clone();
    let params_b = params.clone();

    let inv =
        block(move || crate::db::invitation_depricating::find_by_email(&pool_a, params_a.email)).await?;

    if inv.email_confirmed {
        let new_user = User {
            user_id: Uuid::new_v4(),
            account_id: Uuid::new_v4(),
            email: inv.email.clone(),
            password: hash(&params_b.password),
        };
        println!("new user id {:?}",&new_user.user_id);
        println!("new account id {:?}",&new_user.account_id);

        if api::email::email_is_unique(&inv.email, pool_c).await? {
            let new_account = Account::from(new_user.clone());
            println!("new account id {:?}",&new_account.account_id);
            
            api::email::add_email(&inv.email, pool.clone()).await?;
    
            println!("Sub Domain created, {}", request_subdomain(client, new_account
                .domains_configuration
                .subdomain
                .clone()
                .to_string()).await?);
    
            let user = block(move || create(&pool, new_user.into(), new_account.into())).await?;
            block(move || db::invitation_depricating::remove(&pool_b, &inv.id)).await?;
            respond_json(user.into())
        } else {
            Err(ApiError::BadRequest("Account email already claimed. Restoration not yet build".to_string()))
        }
    } else {
        Err(ApiError::BadRequest(
            "Invitation Not Verified, Check Your Email".into(),
        ))
    }
}

// pub async fn delete_all_users(
//     pool: Data<PgPool>,
// ) -> Result<HttpResponse, ApiError> {
//     use crate::schema::users::dsl::users;
//     use crate::schema::emails::dsl::emails;
//     let conn= pool.get()?;
//     block(move || crate::diesel::delete(users).execute(&conn)).await?;
//     block(move || crate::diesel::delete(emails).execute(&conn)).await?;
//     respond_ok()
// }



