use crate::utils::errors::ApiError;
use actix_web::{
    body::Body,
    web::{HttpResponse, Json},
};
use serde::Serialize;

/// Helper function to reduce boilerplate of an OK/Json response
pub fn respond_json<T>(data: T) -> Result<Json<T>, ApiError>
where
    T: Serialize,
{
    Ok(Json(data))
}

/// Helper function to reduce boilerplate of an empty OK response
pub fn respond_ok() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body(Body::Empty))
}

pub fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(actix_web::http::header::LOCATION, location)
        .finish()
}
