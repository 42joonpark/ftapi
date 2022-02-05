use crate::error::SessionError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenInfo {
    #[serde(rename = "resource_owner_id")]
    pub resource_owner_id: Option<i64>,

    #[serde(rename = "scopes")]
    pub scopes: Option<Vec<String>>,

    #[serde(rename = "expires_in_seconds")]
    pub expires_in_seconds: Option<i64>,

    #[serde(rename = "application")]
    pub application: Option<Application>,

    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Application {
    #[serde(rename = "uid")]
    pub uid: Option<String>,
}

pub async fn token_info(token: &str) -> Result<TokenInfo, SessionError> {
    let url = format!(
        "https://api.intra.42.fr/oauth/token/info?access_token={}",
        token
    );
    let resp = reqwest::get(&url).await?;
    let token_info: TokenInfo = resp.json().await?;
    Ok(token_info)
}

pub async fn check_token_valide(token: &str) -> Result<bool, SessionError> {
    let token_info = token_info(token).await?;
    if token_info.expires_in_seconds.is_none() {
        return Ok(false);
    }
    Ok(true)
}

#[tokio::test]
async fn token_info_fail_test() {
    let res = token_info("not working token").await;
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
    let res = check_token_valide("not working token").await;
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
