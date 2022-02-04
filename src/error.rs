use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error("Error: NoneError")]
    NoneError,
    #[error("Error: Server Unauthorized")]
    ServerUnauthorized,
}
