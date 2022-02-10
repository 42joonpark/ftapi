pub mod results;
pub mod token;

use crate::token::{check_token_valide, generate_token, generate_token_credentials};
use log::{self, debug, warn};
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use std::fs;
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
    #[error("Error: Configure file not found")]
    ConfigFileNotFound,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Session {
    client_id: String,
    client_secret: String,
    login: String,
    access_token: Option<String>,
}

impl Session {
    pub fn new(path: &str) -> Result<Self, SessionError> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub async fn generate_token(&mut self) -> Result<(), SessionError> {
        self.access_token = Some(generate_token_credentials(self.clone()).await?);
        Ok(())
    }

    pub fn get_login(&self) -> &str {
        self.login.as_str()
    }
    pub fn get_client_id(&self) -> &str {
        self.client_id.as_str()
    }
    pub fn get_client_secret(&self) -> &str {
        self.client_secret.as_str()
    }
    pub fn get_access_token(&self) -> Option<String> {
        self.access_token.clone()
    }
    pub fn set_access_token(&mut self, token: String) {
        self.access_token = Some(token);
    }
    pub fn update_access_token(&mut self, token: String) {
        self.access_token = Some(token);
    }
}

impl Session {
    pub async fn call_credentials(&mut self, uri: &str) -> Result<String, SessionError> {
        if self.access_token.is_none() {
            self.access_token = Some(generate_token_credentials(self.clone()).await?);
        }
        let ac_token = self.access_token.clone().unwrap_or_default();
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", self.get_client_id()),
        ];
        let response = client
            .get(format!("{}", uri))
            .header(AUTHORIZATION, format!("Bearer {}", ac_token))
            .form(&params)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                debug!("call(): reqwest OK");
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                warn!("call(): unauthorized");
                return Err(SessionError::UnauthorizedServerError);
            }
            reqwest::StatusCode::FORBIDDEN => {
                warn!("call(): 402 FORBIDDEN ACCESS");
                return Err(SessionError::Fobidden);
            }
            reqwest::StatusCode::NOT_FOUND => {
                warn!("404 NOT FOUND");
                return Err(SessionError::NotFound);
            }
            _ => {
                panic!("uh oh! something unexpected happened");
            }
        }
        let tmp = response.text().await?;
        Ok(tmp)
    }

    pub async fn call(&mut self, uri: &str) -> Result<String, SessionError> {
        if !(check_token_valide(self.get_access_token()).await?) {
            let token = generate_token(self.clone()).await?;
            self.set_access_token(token);
            if let Ok(false) = check_token_valide(self.get_access_token()).await {
                println!("Token is not valid, please check access token.");
                return Err(SessionError::TokenNotValid);
            }
        }
        let ac_token = self.get_access_token().unwrap_or_default();
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", self.get_client_id()),
        ];
        let response = client
            .get(format!("https://api.intra.42.fr/{}", uri))
            .header(AUTHORIZATION, format!("Bearer {}", ac_token))
            .form(&params)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                debug!("call(): reqwest OK");
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                warn!("call(): unauthorized");
                return Err(SessionError::UnauthorizedServerError);
            }
            reqwest::StatusCode::FORBIDDEN => {
                warn!("call(): 402 FORBIDDEN ACCESS");
                return Err(SessionError::Fobidden);
            }
            reqwest::StatusCode::NOT_FOUND => {
                warn!("404 NOT FOUND");
                return Err(SessionError::NotFound);
            }
            _ => {
                panic!("uh oh! something unexpected happened");
            }
        }
        let tmp = response.text().await?;
        Ok(tmp)
    }
}
