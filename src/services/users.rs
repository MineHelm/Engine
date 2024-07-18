use actix_web::http::StatusCode;
use anyhow::Context;
use sqlx::PgPool;

use crate::error::{ApiError, Result};

use super::keys::{ApiKey, KeyTags, KeysService};

pub(crate) struct UsersService;

#[derive(serde::Deserialize, schemars::JsonSchema, apistos::ApiComponent)]
pub struct CreateUserPayload {
    pub username: String,
    pub password: String
}

#[derive(serde::Serialize, schemars::JsonSchema, apistos::ApiComponent)]
pub(crate) struct CreatedUser {
    pub id: i32,
    pub key: ApiKey
}

impl UsersService {
    pub(crate) async fn create_user(
        pool: &PgPool,
        payload: &CreateUserPayload,
        tags: KeyTags
    ) -> Result<CreatedUser> {
        if payload.username.len() > 24 {
            Err(ApiError::message(
                StatusCode::BAD_REQUEST,
                "Username is too long. (max 24 characters)")
            )?
        }

        let password_hash = pwhash::bcrypt::hash(&payload.password)
            .context("Failed to hash password for user creation.")?;
        let key = KeysService::create_new_key();

        let user_id: i32 = sqlx::query_scalar(r###"
            WITH created_user AS (
                INSERT INTO users (username, password)
                VALUES ($1, $2)
                RETURNING id
            ) INSERT INTO keys (id, tags, ownerId)
            VALUES ($3, $4, (SELECT id FROM created_user))
            RETURNING ownerId
        "###)
            .bind(&payload.username)
            .bind(&password_hash)
            .bind(key.as_str())
            .bind(tags.bits())
            .fetch_one(pool)
            .await?;

        log::info!("Created user with id {user_id}");

        Ok(CreatedUser {
            id: user_id,
            key
        })
    }
}
