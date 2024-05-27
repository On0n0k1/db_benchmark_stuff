use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use std::fmt::Debug;

/// Error to be returned by server functions. Since it implements ResponseError,
/// there is no need to manually handle the errors in each API method.
#[derive(Debug)]
pub enum Error {
    #[allow(unused)]
    Other(String),
    Sqlx(sqlx::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match &self {
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let msg = match &self {
            Self::Other(err) => format!("Another error has occurred: {err:?}"),
            Self::Sqlx(err) => format!("Database error has occurred: {err:?}"),
        };
        HttpResponse::build(self.status_code()).body(msg)
    }
}

impl Error {
    /// Simplify error handling when using `.or_else`
    pub fn sqlx<A>(err: sqlx::Error) -> Result<A, Error> {
        Err(Self::Sqlx(err))
    }
}
