use crate::serde_json::Value;
use crate::utils::authentication::{decode_jwt, PrivateClaim};
use crate::utils::database::PgPool;
use crate::utils::errors::ApiError;
use crate::utils::helpers::respond_json;
use actix_identity::Identity;
use actix_web::web::{Bytes, Data, Json, Path};
use actix_web::{HttpMessage, HttpRequest, HttpResponse};

pub async fn replicate(
    req: HttpRequest,
    body: Bytes,
    // payload: Json<Value>,
    id: Identity,
    database_name: Path<String>,
) -> Result<HttpResponse, ApiError> {
    let client = actix_web::client::Client::new();
    let forwarded_req = client
        .request_from(
            format!(
                "localhost:5984/{}_replicate?name=couched_visits&password=uX2b6@q5CxOjT7NrxYDc",
                database_name.into_inner()
            )
            .as_str(),
            req.head(),
        )
        .no_decompress();

    let mut res = forwarded_req.send_body(body).await?;

    let mut client_resp = HttpResponse::build(res.status());

    for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    Ok(client_resp.body(res.body().await?))
}
