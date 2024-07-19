use actix_web::{cookie::Cookie, http::StatusCode, web::{Data, Json}, HttpResponse};
use apistos::{api_operation, web::{self, resource, Scope}};
use serde_json::json;

use crate::{config::MHConfig, database::DB, error::{ApiError, Result}, services::{keys::KeyTags, users::CreateUserPayload, UsersService}};

pub fn handlers() -> Scope {
    web::scope("/onboarding")
        .service(resource("/check").route(web::get().to(handle_onboarding_check)))
        .service(resource("/create_admin").route(web::post().to(handle_create_admin)))
        .service(resource("/select_engine").route(web::post().to(handle_select_engine)))
        .service(resource("/finish").route(web::post().to(handle_finish_onboarding)))
}

#[api_operation(
    tag = "onboarding",
    summary = "Finishes onboarding",
    description = "This finishes onboarding and restarts the whole API.",
)]
async fn handle_finish_onboarding(
    config: Data<MHConfig>,
) -> Result<HttpResponse> {
    config.update(|cfg| cfg.is_onboarded = true);
    // TODO: restart instead of killing program
    std::process::exit(1)
}

#[derive(serde::Deserialize, schemars::JsonSchema, apistos::ApiComponent)]
pub struct SelectEnginePayload {
    engine: String
}

#[api_operation(
    tag = "onboarding",
    summary = "Selects an engine",
    description = "Selects an engine to use for MineHelm. This WILL break existing instances.",
)]
async fn handle_select_engine(
    payload: Json<SelectEnginePayload>,
    config: Data<MHConfig>,
) -> Result<HttpResponse> {
    if config.read().is_onboarded {
        Err(ApiError::message(StatusCode::BAD_REQUEST, "Onboarding is already finished."))?
    }

    if payload.engine != "docker" && payload.engine != "kubernetes" {
        Err(ApiError::message(
            StatusCode::BAD_REQUEST, 
            format!("'{}' is not a valid engine.", payload.engine)
        ))?
    }

    config.update(|cfg| cfg.engine = match payload.engine.as_str() {
        "docker" => crate::engine::ContainerEngineKind::Docker,
        "kubernetes" => crate::engine::ContainerEngineKind::Kubernetes,
        _ => unreachable!()
    });

    Ok(HttpResponse::Ok().json(json!({
        "engine": payload.engine
    })))
}

#[api_operation(
    tag = "onboarding",
    summary = "Creates FIRST admin user",
    description = "Creates first admin user. This only works when app is not onboarded yet.",
)]
async fn handle_create_admin(
    payload: Json<CreateUserPayload>,
    db: Data<DB>,
    config: Data<MHConfig>
) -> Result<HttpResponse> {
    if config.read().is_onboarded {
        Err(ApiError::message(StatusCode::BAD_REQUEST, "Onboarding is already finished."))?
    }

    let created_user = UsersService::create_user(db.pool(), &payload, KeyTags::ADMIN)
        .await?;

    Ok(HttpResponse::Ok()
        .cookie(
            Cookie::build("API_KEY", created_user.key.as_str())
                .http_only(true)
                .secure(true)
                .permanent()
                .finish()
        )
        .json(created_user)
    )
}

#[api_operation(
    tag = "onboarding",
    summary = "Checks whether app onboarding should be done",
    description = "This checks whether app configuration and first user are created.",
)]
async fn handle_onboarding_check(
    config: Data<MHConfig>
) -> HttpResponse {
    if !config.read().is_onboarded {
        HttpResponse::Ok().json(json!({
            "shouldOnboard": true,
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "shouldOnboard": false
        }))
    }
}
