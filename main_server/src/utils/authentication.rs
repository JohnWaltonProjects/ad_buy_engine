use crate::utils::errors::ApiError;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use ad_buy_engine::chrono::{Duration, Utc};
use ad_buy_engine::uuid::Uuid;
use argon2rs::argon2i_simple;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PrivateClaim {
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub email: String,
    exp: i64,
}

impl PrivateClaim {
    pub fn new(user_id: Uuid, account_id: Uuid, email: String) -> Self {
        Self {
            user_id,
            account_id,
            email,
            exp: (Utc::now()
                + Duration::hours(
                    std::env::var("JWT_EXPIRATION")
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                ))
            .timestamp(),
        }
    }
}

pub fn create_jwt(private_claim: PrivateClaim) -> Result<String, ApiError> {
    let encoding_key = EncodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    encode(&Header::default(), &private_claim, &encoding_key)
        .map_err(|e| ApiError::CannotEncodeJwtToken(e.to_string()))
}

pub fn decode_jwt(token: &str) -> Result<PrivateClaim, ApiError> {
    let decoding_key = DecodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    decode::<PrivateClaim>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ApiError::CannotDecodeJwtToken(e.to_string()))
}

pub fn hash(password: &str) -> String {
    argon2i_simple(&password, &CONFIG.auth_salt)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

pub fn get_identity_service() -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(&CONFIG.session_key.as_ref())
            .name(&CONFIG.session_name)
            .max_age_time(ad_buy_engine::time::Duration::days(1))
            .secure(true),
    )
}
