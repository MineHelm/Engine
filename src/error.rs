use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use apistos::ApiErrorComponent;
use serde_json::json;

#[derive(thiserror::Error, Debug, ApiErrorComponent)]
#[openapi_error(
    status(code = 500),
    status(code = 500),
    status(code = 400),
    status(code = 500)
)]
pub enum ApiError {
    #[error("An unspecified internal error has occured: {0}")]
    Generic(#[from] anyhow::Error),
    #[error("Database error has occured: {0}")]
    Database(#[from] sqlx::Error),
    #[error("{1}")]
    Message(StatusCode, String),
    #[error("Docker error occured: {0}")]
    Docker(#[from] docker_api::Error),
}

impl ApiError {
    pub fn message(code: StatusCode, msg: impl AsRef<str>) -> Self {
        Self::Message(code, msg.as_ref().to_string())
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::Message(code, _) => *code,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(json!({
            "error": self.to_string()
        }))
    }
}

pub(crate) type Result<T> = std::result::Result<T, ApiError>;
