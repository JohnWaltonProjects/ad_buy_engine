use actix_cors::Cors;
use actix_files::Files;
use actix_ratelimit::MemoryStore;
use actix_redis::RedisActor;
use actix_web::middleware::Logger;
use actix_web::http::header;
use actix_web::web::{get, post, resource, scope, JsonConfig};
use ad_buy_engine::chrono::Utc;
use ad_buy_engine::constant::local_system_location::DIRECTORY_LOCATION_MAIN_PUBLIC_STATIC;
use ad_buy_engine::data::backend_models::campaign::CampaignModel;
use ad_buy_engine::data::elements::campaign::Campaign;
use ad_buy_engine::diesel::prelude::*;
use ad_buy_engine::diesel::PgConnection;
use awc::Client;
use r2d2_diesel::ConnectionManager;
use actix_web::{HttpServer, App};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn server() -> std::io::Result<()> {
    ad_buy_engine::dotenv::dotenv().ok();
    ad_buy_engine::env_logger::init();

    // let pool = establish_connection();
    // diesel_migrations::run_pending_migrations(&pool.clone().get().expect("hyuu"))
    //     .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // let cache = if let Ok(var) = std::env::var("REDIS_URL") {
    //     let cache = RedisActor::start(&var);
    //     cache
    // } else {
    //     panic!("Redis URL env var not found")
    // };
    // let res = load_click_identities_for_cache(&pool).expect("g56tFDS");

    // println!("click identities number: {}", &res.len());
	//
    // res.into_iter().map(|s| create_click_identity(s, &cache));

    // let mut filtered_restored: Vec<Campaign> = {
    //     use crate::schema::campaigns::dsl::campaigns;
    //     campaigns
    //         .load::<CampaignModel>(&pool.clone().get().expect("4rgfsadf"))
    //         .unwrap()
    //         .iter()
    //         .cloned()
    //         .map(|s| s.into())
    //         .collect::<Vec<Campaign>>()
    // };
	//
    // filtered_restored
    //     .iter()
    //     .filter(|s| s.last_clicked.timestamp() < (Utc::now().timestamp() + 200_000));
    // println!("campagin in appstate: {}", &filtered_restored.len());

    // let app_state = init_state(filtered_restored);
    let store = MemoryStore::new();
    // let campaign_agent = CampaignAgent::start("campaign_server:1488");
    let couch = ad_buy_engine::couch_rs::Client::new(
        // "http://127.0.0.1:5984/",
        "http://couch_database:5984",
        "couched_visits",
        "uX2b6@q5CxOjT7NrxYDc",
    )
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let server = HttpServer::new(move || {
        App::new()
            // .wrap(RedirectSchemeBuilder::new().enable(true).build())
            // .configure(add_cache)
            .wrap(
	            Cors::default()
		            .allow_any_method()
		            .allow_any_header()
		            .max_age(3600)
		
		            // // add specific origin to allowed origin list
		            // .allowed_origin("http://project.local:8080")
		            // // allow any port on localhost
		            // .allowed_origin_fn(|origin, _req_head| {
			        //     origin.as_bytes().starts_with(b"http://localhost")
		            //
			        //     // manual alternative:
			        //     // unwrapping is acceptable on the origin header since this function is
			        //     // only called when it exists
			        //     // req_head
			        //     //     .headers()
			        //     //     .get(header::ORIGIN)
			        //     //     .unwrap()
			        //     //     .as_bytes()
			        //     //     .starts_with(b"http://localhost")
		            // })
		            // // set allowed methods list
		            // .allowed_methods(vec!["GET", "POST"])
		            // // set allowed request header list
		            // .allowed_headers(&[header::AUTHORIZATION, header::ACCEPT])
		            // // add header to allowed list
		            // .allowed_header(header::CONTENT_TYPE)
		            // // set list of headers that are safe to expose
		            // .expose_headers(&[header::CONTENT_DISPOSITION])
		            // // set CORS rules ttl
		            // .max_age(3600),
	                
	                // Cors::new()
                //     .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                //     .allowed_header(header::CONTENT_TYPE)
                //     .max_age(3600)
                //     .finish(),
            )
            .wrap(Logger::default())
            .data(JsonConfig::default().limit(4_096_000))
            .wrap(get_identity_service())
            .data(Client::new())
            // .data(pool.clone())
            // .data(cache.clone())
            .data(couch.clone())
            // .app_data(app_state.clone())
            // .wrap_fn(|req, srv| {
            //     println!("\n");
            //     srv.call(req).map(|res| res)
            // })
            // .service(resource("/couch").route(get().to(test_create_couch_database)))
            // .service(resource("/extra/{num}").route(post().to(extra_multiple)))
            // .service(resource("/extra").route(get().to(extra_single)))
            // .service(resource("/learn/{campaign_id}").route(get().to(process_initial_click)))
            // .service(resource("/action").route(post().to(unimplemented!())))
            // .configure(public_routes)
            // .configure(private_routes)
            .service(
                scope("").default_service(
                    Files::new("", DIRECTORY_LOCATION_MAIN_PUBLIC_STATIC)
                        .index_file("index.html")
                        .use_last_modified(true),
                ),
            )
        // .data(campaign_agent.clone())
    })
    .bind("campaign_server:80")?
    // .bind_openssl("campaign_server:443", ssl_config())?
    .workers(1)
    .run();

    server.await
}
