use actix_web::web::{block, Data, HttpResponse, Json, Path, Query};
use ad_buy_engine::data::account::Account;
use ad_buy_engine::data::user::User;
use ad_buy_engine::diesel::prelude::*;
use ad_buy_engine::serde::Serialize;
use ad_buy_engine::uuid::Uuid;
use ad_buy_engine::{CreateUserRequest, UserResponse};

use crate::db;
use crate::db::user_depricating::*;
use crate::dns::dns_cname::request_subdomain;
use crate::management;
use crate::management::api;
use crate::management::couch::create_couch_database;
use crate::schema::accounts::dsl::{accounts, id as account_id};
use crate::schema::invitation::dsl::invitation;
use crate::schema::users::dsl::{id as user_id, users};
use crate::utils::authentication::hash;
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use crate::utils::helpers::{redirect_to, respond_json, respond_ok};
use std::collections::HashMap;

pub async fn create_user(
    pool: Data<PgPool>,
    params: Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let pool_a = pool.clone();
    let pool_b = pool.clone();
    let pool_c = pool.clone();
    let params_a = params.clone();
    let params_b = params.clone();

    let inv =
        block(move || crate::db::invitation_depricating::find_by_email(&pool_a, params_a.email))
            .await?;

    if inv.email_confirmed {
        let new_user = User {
            user_id: Uuid::new_v4(),
            account_id: Uuid::new_v4(),
            email: inv.email.clone(),
            password: hash(&params_b.password),
        };
        println!("new user id {:?}", &new_user.user_id);
        println!("new account id {:?}", &new_user.account_id);

        if api::email::email_is_unique(&inv.email, pool_c).await? {
            let new_account = Account::from(new_user.clone());
            println!("new account id {:?}", &new_account.account_id);

            api::email::add_email(&inv.email, pool.clone()).await?;

            println!(
                "Sub Domain created, {}",
                request_subdomain(
                    new_account
                        .domains_configuration
                        .subdomain
                        .clone()
                        .to_string()
                )
                .await?
            );

            let user_details = CouchUserDetails::create(new_account.account_id.to_string());

            let username = create_couch_database(user_details).await?;

            let user = block(move || create(&pool, new_user.into(), new_account.into())).await?;
            block(move || db::invitation_depricating::remove(&pool_b, &inv.id)).await?;
            respond_json(user.into())
        } else {
            Err(ApiError::BadRequest(
                "Account email already claimed. Restoration not yet build".to_string(),
            ))
        }
    } else {
        Err(ApiError::BadRequest(
            "Invitation Not Verified, Check Your Email".into(),
        ))
    }
}

pub struct CouchUserDetails {
    pub username: String,
    pub password: String,
    pub database_name: String,
}

impl CouchUserDetails {
    pub fn create(_account_id: String) -> CouchUserDetails {
        let slim_account_id = _account_id
            .chars()
            .filter(|s| *s != '-')
            .collect::<String>();
        let password_hash = hash(&slim_account_id);

        Self {
            username: slim_account_id.clone(),
            password: password_hash,
            database_name: slim_account_id,
        }
    }
}
