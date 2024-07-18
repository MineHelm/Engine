use actix_web::{web::Data, HttpResponse};
use anyhow::Context;
use apistos::{api_operation, web::{self, Scope}};
use serde_json::json;

use crate::{database::DB, error::Result};

pub fn handlers() -> Scope {
    web::scope("/onboarding")
        .route("/check", web::get().to(handle_onboarding_check))
}

#[api_operation(
    tag = "onboarding",
    summary = "Checks whether app onboarding should be done",
    description = "This checks whether app configuration and first user are created.",
)]
async fn handle_onboarding_check(
    db: Data<DB>
) -> Result<HttpResponse> {
    if should_onboard(&db).await? {
        Ok(HttpResponse::Ok().json(json!({
            "shouldOnboard": true,
        })))
    } else {
        Ok(HttpResponse::Ok().json(json!({
            "shouldOnboard": false
        })))
    }
}

async fn should_onboard(
    db: &Data<DB>
) -> Result<bool> {
    let users_count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(db.pool())
        .await.context("Failed to check user count for /onboarding/check")?;

    Ok(users_count == 0)
}
