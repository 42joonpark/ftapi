use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Application {
    #[serde(rename = "uid")]
    pub uid: Option<String>,
}

pub async fn get_token_info(token: &str) -> Result<TokenInfo, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.intra.42.fr/oauth/token/info?access_token={}",
        token
    );
    let resp = reqwest::get(&url).await?;
    let token_info: TokenInfo = resp.json().await?;
    Ok(token_info)
}

pub async fn check_token_valide(token: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let token_info = get_token_info(token).await?;
    if token_info.expires_in_seconds.is_none() {
        return Ok(false);
    }
    Ok(true)
}
