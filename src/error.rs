use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ParseUrlError(#[from] url::ParseError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error("Error: toml Error")]
    TomlError(#[from] toml::de::Error),
    #[error("Error: Not valide token Error")]
    TokenNotValid,
    #[error("Error: NoneError")]
    NoneError,
    #[error("Error: Server Unauthorized")]
    UnauthorizedServerError,
    #[error("Error: 403 Fobidden Access")]
    Fobidden,
    #[error("Error: 404 Page or resource is not found")]
    NotFound,
}
