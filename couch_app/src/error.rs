use actix_web::body::{Body, MessageBody};
use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::{BaseHttpResponse, HttpResponse};
use ad_buy_engine::couch_rs::error::CouchError;
use std::fmt::{Display, Formatter};
use std::pin::Pin;

// #[derive(Debug, Clone)]
// pub struct CouchAppError {
//     msg: String,
//     status: StatusCode,
// }
//
// impl Display for CouchAppError {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         f.write_str(&self.msg)
//     }
// }
//
// impl From<CouchError> for CouchAppError {
//     fn from(err: CouchError) -> Self {
//         Self {
//             msg: err.message,
//             status: err.status,
//         }
//     }
// }
//
// impl ResponseError for CouchAppError {
//     fn error_response(&self) -> BaseHttpResponse<Body> {
//         BaseHttpResponse::new(self.status)
//             .set_body(Body::Message(Pin::new(Box::new(self.msg.clone()))))
//     }
// }
