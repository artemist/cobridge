use axum::response::IntoResponse;
use hyper::StatusCode;
use std::fmt::Display;
use tracing::error;

pub type ResponseResult<T> = std::result::Result<T, ResponseError>;
pub struct ResponseError(anyhow::Error);

impl From<anyhow::Error> for ResponseError {
    fn from(err: anyhow::Error) -> Self {
        Self(err)
    }
}

impl From<ErrorWithStatus> for ResponseError {
    fn from(err: ErrorWithStatus) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        match self.0.downcast::<ErrorWithStatus>() {
            Ok(err) => err.into_response(),
            Err(err) => {
                error!(
                    "internal server error. chain:\n{}",
                    err.chain()
                        .map(|cause| cause.to_string())
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            }
        }
    }
}

#[derive(Debug)]
pub struct ErrorWithStatus {
    pub status: StatusCode,
    pub message: String,
}

impl Display for ErrorWithStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ErrorWithStatus {}

impl IntoResponse for ErrorWithStatus {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.message).into_response()
    }
}
