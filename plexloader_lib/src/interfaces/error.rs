use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkResponseError {
    #[error("unauthorized")]
    UnAuthorized,
    #[error("unknown error")]
    UnknownError(#[from] ureq::Error),
    #[error("can't serialize to json")]
    SerializationError(#[from] serde_json::Error)
}
