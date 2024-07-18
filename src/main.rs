use std::{error::Error, net::Ipv4Addr};

use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, Responder};
use apistos::{api_operation, app::{BuildConfig, OpenApiWrapper}, info::Info, spec::Spec, web, SwaggerUIConfig};
use sqlx::postgres::PgPoolOptions;

use database::DB;

pub(crate) mod error;
pub(crate) mod database;
pub(crate) mod services;
mod routes;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    std::env::set_var("RUST_LOG", "info");
    dotenvy::dotenv().expect("Failed to load .env");
    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set"))
        .await.expect("Failed to connect to the database");

    sqlx::migrate!()
        .run(&pool)
        .await.expect("Failed to migrate database");

    let db = DB::new(pool);

    log::info!("Starting MineHelm Engine on port 7241");
    HttpServer::new(move || {
        // Swagger docs spec
        let spec = Spec {
            info: Info {
                title: "MineHelm Engine API Docs".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        App::new()
            .document(spec)
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(db.clone()))
            .route("/ping", web::get().to(ping_handler))
            .service(routes::handlers())
            .build_with(
                "/openapi.json",
                BuildConfig::default()
                    .with(SwaggerUIConfig::new(&"/apidoc"))
            )
    })
    .bind((Ipv4Addr::UNSPECIFIED, 7241))?
    .run()
    .await
}

#[api_operation(
    tag = "utility",
    summary = "Replies with 'Pong!'",
    description = "This can be used to check whether the engine is active and available.",
)]
async fn ping_handler() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}
