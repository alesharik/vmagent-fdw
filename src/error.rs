use pgrx::pg_sys::panic::ErrorReport;
use pgrx::PgSqlErrorCode;

#[derive(Debug)]
pub enum Error {
    AddressOptionRequired,
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
}

impl From<Error> for ErrorReport {
    fn from(value: Error) -> Self {
        match value {
            Error::AddressOptionRequired => ErrorReport::new(
                PgSqlErrorCode::ERRCODE_FDW_ERROR,
                "Address option required",
                "",
            ),
            Error::SerdeError(e) => ErrorReport::new(
                PgSqlErrorCode::ERRCODE_FDW_ERROR,
                format!("Serde error: {}", e),
                "",
            ),
            Error::ReqwestError(e) => ErrorReport::new(
                PgSqlErrorCode::ERRCODE_FDW_ERROR,
                format!("Reqwest error: {}", e),
                "",
            ),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::ReqwestError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::SerdeError(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
