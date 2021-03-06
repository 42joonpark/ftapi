use crate::Session;
use crate::SessionError;
use log::{self, debug};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenInfo {
    #[serde(rename = "resource_owner_id")]
    pub resource_owner_id: Option<i64>,

    #[serde(rename = "scopes")]
    pub scopes: Option<Vec<String>>,

    #[serde(rename = "expires_in_seconds")]
    pub expires_in_seconds: Option<i64>,

    #[serde(rename = "application")]
    application: Option<Application>,

    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Application {
    #[serde(rename = "uid")]
    uid: Option<String>,
}

pub async fn token_info(token: Option<String>) -> Result<TokenInfo, SessionError> {
    let url = format!(
        "https://api.intra.42.fr/oauth/token/info?access_token={}",
        token.unwrap_or_default()
    );
    let resp = reqwest::get(&url).await?;
    let token_info: TokenInfo = resp.json().await?;
    Ok(token_info)
}

pub async fn check_token_valide(token: Option<String>) -> Result<bool, SessionError> {
    let token_info = token_info(token).await?;
    if token_info.expires_in_seconds.is_none() {
        return Ok(false);
    }
    Ok(true)
}

#[tokio::test]
async fn token_info_fail_test() {
    let res = token_info(Some("not working token".to_string())).await;
    // let res = token_info(None).await;
    if let Ok(token_info) = res {
        println!("{:?}", token_info); // cargo run test -- --nocapture
        assert_eq!(token_info.application.is_none(), true);
    }
}

/*
#[tokio::test]
async fn token_info_success_test() {
    let res =
        token_info("Some Working Token")
            .await;
    if let Ok(token_info) = res {
        println!("{:?}", token_info);
        assert_eq!(token_info.application.is_none(), false);
    }
}
*/

#[tokio::test]
async fn check_token_valide_fail_test() {
    let res = check_token_valide(Some("not working token".to_string())).await;
    // let res = check_token_valide(None).await;
    if let Ok(t) = res {
        assert_eq!(t, false);
    }
}

/*
#[tokio::test]
async fn check_token_valide_success_test() {
    let res = check_token_valide(
        "Some Working Token",
    )
    .await;
    if let Ok(t) = res {
        assert_eq!(t, true);
    }
}
*/

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
