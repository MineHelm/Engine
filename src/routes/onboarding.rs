use actix_web::{http::StatusCode, web::{Data, Json}, HttpResponse};
use apistos::{api_operation, web::{self, Scope}};
use serde_json::json;

use crate::{config::MHConfig, database::DB, error::{ApiError, Result}, services::{keys::{ApiKey, KeyTags}, users::CreateUserPayload, UsersService}};

pub fn handlers() -> Scope {
    web::scope("/onboarding")
        .route("/check", web::get().to(handle_onboarding_check))
        .route("/create_admin", web::post().to(handle_create_admin))
        .route("/select_engine", web::post().to(handle_select_engine))
        .route("/finish", web::post().to(handle_finish_onboarding))
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
    std::process::exit(0)
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

    Ok(HttpResponse::Ok().json(
        created_user
    ))
}

#[api_operation(
    tag = "onboarding",
    summary = "Checks whether app onboarding should be done",
    description = "This checks whether app configuration and first user are created.",
)]
async fn handle_onboarding_check(
    config: Data<MHConfig>
) -> Result<HttpResponse> {
    if !config.read().is_onboarded {
        Ok(HttpResponse::Ok().json(json!({
            "shouldOnboard": true,
        })))
    } else {
        Ok(HttpResponse::Ok().json(json!({
            "shouldOnboard": false
        })))
    }
}
