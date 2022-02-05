use crate::error::SessionError;
use serde::Deserialize;
use std::fs;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Session {
    client_id: String,
    client_secret: String,
    access_token: Option<String>,
}

impl Session {
    pub fn new() -> Result<Self, SessionError> {
        let path = "./config.toml";
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
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

impl Session {}
