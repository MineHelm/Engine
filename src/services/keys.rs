use std::future::{ready, Ready};

use actix_web::{http::StatusCode, FromRequest};
use apistos::ApiSecurity;
use bitflags::bitflags;
use rand::{distributions::Alphanumeric, Rng};

use crate::error::ApiError;

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema, Debug, Clone, PartialEq, Eq, Hash, ApiSecurity)]
#[openapi_security(scheme(security_type(api_key(name = "api_key", api_key_in = "header"))))]
#[repr(transparent)]
pub(crate) struct ApiKey(String);

impl ApiKey {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromRequest for ApiKey {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(header) = req.headers().get("X-Api-Key") {
            ready(Ok(
                ApiKey(header.to_str().unwrap().to_string())
            ))
        } else {
            ready(
                Err(ApiError::message(StatusCode::BAD_REQUEST, "Expected X-Api-Key header to be present."))
            )
        }
    }
}

pub(crate) struct KeysService;

impl KeysService {
    pub(crate) fn create_new_key() -> ApiKey {
        ApiKey(
            rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(128)
            .map(char::from)
            .collect::<String>()
        )
    }
}

bitflags! {
    pub struct KeyTags: i32 {
        const ADMIN = 1;
    }
}
