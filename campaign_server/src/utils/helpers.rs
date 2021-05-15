use crate::utils::errors::ApiError;
use actix_web::http::StatusCode;
use actix_web::{
    body::Body,
    web::{HttpResponse, Json},
    BaseHttpResponse,
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
pub fn respond_ok() -> Result<HttpResponse<Body>, ApiError> {
    Ok(HttpResponse::ok())
}

pub fn redirect_to(location: &str) -> HttpResponse<Body> {
    HttpResponse::new(StatusCode::FOUND)
        .header(actix_web::http::header::LOCATION, location)
        .finish()
}
