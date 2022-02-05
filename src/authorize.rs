use log::debug;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use serde::Deserialize;
use std::fs;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;

use crate::error::SessionError;

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
    pub fn get_access_token(&self) -> &str {
        match &self.access_token {
            Some(token) => token,
            None => "",
        }
    }
    pub fn set_access_token(&mut self, token: String) {
        self.access_token = Some(token);
    }
    pub fn update_access_token(&mut self, token: String) {
        self.access_token = Some(token);
    }
}

pub async fn generate_token(session: Session) -> Result<String, SessionError> {
    let client = BasicClient::new(
        ClientId::new(String::from(session.get_client_id())),
        Some(ClientSecret::new(String::from(session.get_client_secret()))),
        AuthUrl::new("https://api.intra.42.fr/oauth/authorize".to_string())?,
        Some(TokenUrl::new(
            "https://api.intra.42.fr/oauth/token".to_string(),
        )?),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string())?);

    let (auth_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("public".to_string()))
        .url();
    println!("Browse to: {}", auth_url);

    let ac_token = local_server(client).await?;
    Ok(ac_token)
}

/// Create local server with port number 8000 and waits for user to finish authorize.
async fn local_server(client: BasicClient) -> Result<String, SessionError> {
    let ac_token;
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&mut stream);
                let mut request_line = String::new();
                reader.read_line(&mut request_line).await?;
                let redirect_url = match request_line.split_whitespace().nth(1) {
                    Some(url) => url,
                    None => return Err(SessionError::NoneError),
                };
                let url = Url::parse(&("http://localhost".to_string() + redirect_url))?;

                let code_pair = match url.query_pairs().find(|pair| {
                    let &(ref key, _) = pair;
                    key == "code"
                }) {
                    Some(code) => code,
                    None => return Err(SessionError::NoneError),
                };

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = match url.query_pairs().find(|pair| {
                    let &(ref key, _) = pair;
                    key == "state"
                }) {
                    Some(state) => state,
                    None => return Err(SessionError::NoneError),
                };

                let (_, value) = state_pair;
                state = CsrfToken::new(value.into_owned());
            }
            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).await?;

            debug!("42API returned the following code:\n{}\n", code.secret());
            debug!("42API returned the following state:\n{}\n", state.secret());

            // Exchange the code with a token.
            let token_res = client
                .exchange_code(code)
                .request_async(async_http_client)
                .await;
            let token = match token_res {
                Err(_) => return Err(SessionError::UnauthorizedServerError),
                Ok(t) => t,
            };
            debug!("42API returned the following token:\n{:?}\n", token);

            let scopes = if let Some(scopes_vec) = token.scopes() {
                scopes_vec
                    .iter()
                    .map(|comma_separated| comma_separated.split(','))
                    .flatten()
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            ac_token = token.access_token().secret().to_owned();
            debug!("Access Token: {:?}", ac_token);
            debug!("42API returned the following scopes:\n{:?}\n", scopes);
            break;
        }
    }
    Ok(ac_token)
}

/*
#[tokio::test]
async fn authorize_test() {
    // Don't forget to test with --nocapture option
    let res = generate_token(Session {
        client_id: "YOUR CLIENT_ID".to_string(),
        client_secret: "YOUR CLIENT SECRET"
            .to_string(),
    })
    .await;
    if let Ok(t) = res {
        assert_ne!(t, "".to_string());
    }
}
*/
