use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use apistos::ApiErrorComponent;
use serde_json::json;

#[derive(thiserror::Error, Debug, ApiErrorComponent)]
#[openapi_error(status(code = 500))]
#[error("An unspecified internal error has occured: {0}")]
pub struct InternalError(#[from] anyhow::Error);

impl ResponseError for InternalError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(json!({
            "error": self.to_string()
        }))
    }
}

pub(crate) type Result<T> = std::result::Result<T, InternalError>;
