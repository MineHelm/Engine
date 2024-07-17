use std::{error::Error, net::Ipv4Addr};

use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, Responder};
use apistos::{api_operation, app::{BuildConfig, OpenApiWrapper}, info::Info, spec::Spec, web, SwaggerUIConfig};

pub(crate) mod error;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();

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

        log::info!("Starting MineHelm Engine on port 7241");

        App::new()
            .document(spec)
            .wrap(Logger::default())
            .route("/ping", web::get().to(ping_handler))
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
