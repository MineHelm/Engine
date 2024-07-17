use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(thiserror::Error, Debug)]
#[error("An unspecified internal error has occured: {0}")]
pub struct InternalError(#[from] anyhow::Error);

impl ResponseError for InternalError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

pub(crate) type Result<T> = std::result::Result<T, InternalError>;
