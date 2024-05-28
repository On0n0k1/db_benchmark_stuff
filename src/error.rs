use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use std::fmt::Debug;

/// Error to be returned by server functions. Since it implements ResponseError,
/// there is no need to manually handle the errors in each API method.
#[derive(Debug)]
pub enum Error {
    #[allow(clippy::upper_case_acronyms)]
    ADO(tiberius::error::Error),
    TCPStream(std::io::Error),
    ClientConnection(tiberius::error::Error),
    Query(tiberius::error::Error),
    IntoRow(tiberius::error::Error),
    NotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match &self {
            Self::ADO(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TCPStream(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ClientConnection(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Query(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::IntoRow(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let msg = match &self {
            Self::ADO(err) => format!("Failed to parse config data: {err:?}"),
            Self::TCPStream(err) => format!("Failed to connect to server: {err:?}"),
            Self::ClientConnection(err) => format!("Failed to connect with client: {err:?}"),
            Self::Query(err) => format!("Failed to query: {err:?}"),
            Self::IntoRow(err) => format!("Converting output into row: {err:?}"),
            Self::NotFound => "Entry not found".to_string(),
        };
        HttpResponse::build(self.status_code()).body(msg)
    }
}

impl Error {
    pub fn ado<A>(err: tiberius::error::Error) -> Result<A, Error> {
        Err(Self::ADO(err))
    }

    pub fn tcp_stream<A>(err: std::io::Error) -> Result<A, Error> {
        Err(Self::TCPStream(err))
    }

    pub fn client_connection<A>(err: tiberius::error::Error) -> Result<A, Error> {
        Err(Self::ClientConnection(err))
    }

    pub fn query<A>(err: tiberius::error::Error) -> Result<A, Error> {
        Err(Self::Query(err))
    }

    pub fn into_row<A>(err: tiberius::error::Error) -> Result<A, Error> {
        Err(Self::IntoRow(err))
    }
}
