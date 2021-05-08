use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::http::header::REFERER;
use actix_web::HttpRequest;
use ad_buy_engine::Url;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use std::collections::HashMap;
use std::str::FromStr;

pub fn ssl_config() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("privkey.pem", SslFiletype::PEM)
        .expect("ho345fd");
    builder
        .set_certificate_chain_file("fullchain.pem")
        .expect("hi53gs");
    builder
}

pub fn rate_limit(
    max_request: usize,
    time_limit: u64,
    store: MemoryStore,
) -> RateLimiter<MemoryStoreActor> {
    RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
        .with_interval(std::time::Duration::from_secs(time_limit))
        .with_max_requests(max_request)
}

// pub fn  ssl_config() -> SslAcceptorBuilder {
//     let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
//     builder
//         .set_private_key_file(
//             "/etc/letsencrypt/live/adbuyengine.com/privkey.pem",
//             SslFiletype::PEM,
//         )
//         .expect("ho345fd");
//     builder
//         .set_certificate_chain_file("/etc/letsencrypt/live/adbuyengine.com/fullchain.pem")
//         .expect("hi53gs");
//     builder
// }

pub mod misc_utils {
    use super::*;

    pub fn to_hashmap_from_str(string: &str) -> Result<HashMap<String, String>, String> {
        let mut hash_map = HashMap::new();
        for keyword in string.split('&') {
            if !keyword.is_empty() {
                let param = keyword.split('=').collect::<Vec<_>>();
                if param.len() < 3 {
                    hash_map.insert(param[0].to_string(), param[1].to_string());
                } else {
                    return Err(format!("param greater than 3"));
                }
            }
        }
        Ok(hash_map)
    }
}

pub mod http_request_functions {
    use super::*;
    use crate::helper_functions::misc_utils::to_hashmap_from_str;
    use actix_web::web::Query;
    use uuid::Uuid;

    // pub fn extract_ip(req: &HttpRequest) -> Result<Uuid, String> {
    //     
    // }
    // pub fn extract_user_agent(req: &HttpRequest) -> Result<String, String> {
    //     
    // }
    pub fn extract_matrix_id(req: &HttpRequest) -> Result<Uuid, String> {
        let referrer = extract_referrer_url(req)?;
        if let Some(query) = referrer.query() {
            let query_hashmap = to_hashmap_from_str(query)?;
            if let Some(extracted_matrix_id) = query_hashmap.get("mid") {
                if let Ok(id) = Uuid::from_str(&extracted_matrix_id) {
                    Ok(id)
                } else {
                    return Err(format!("Uuid parse failed: Y%$G"));
                }
            } else {
                return Err(format!("No extracted matrix id found"));
            }
        } else {
            Err(format!("No query found: $TYHG"))
        }
    }

    pub fn extract_referrer_url(req: &HttpRequest) -> Result<Url, String> {
        if let Some(referrer) = req.headers().get(REFERER) {
            if let Ok(referrer_str) = referrer.to_str() {
                if let Ok(url) = Url::parse(referrer_str) {
                    Ok(url)
                } else {
                    Err(format!("url from str parse failed"))
                }
            } else {
                Err(format!("referrer to str failed"))
            }
        } else {
            Err(format!("No referrer url found:GHT$%^"))
        }
    }
}
