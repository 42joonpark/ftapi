pub mod cli;
pub mod error;
pub mod token;
pub mod authorize;
use reqwest::header::AUTHORIZATION;
use authorize::{Session, generate_token};
use token::{TokenInfo, check_token_valide};
use cli::{Config, list_available_commands};
use log::{self, debug, info, warn};
use error::SessionError;

async fn run(prog: &mut Program, config: Config) -> Result<(), SessionError> {
    let command = config.command.to_owned();
    // let cmd = command.trim().to_uppercase();
    let res = prog.call("v2/me").await?;
    println!("{:#?}", res);
    Ok(())
}

struct Program {
    session: Session,
    access_token: Option<String>,
    token: Option<TokenInfo>,
}

impl Program {
    pub fn get_access_token(&self) -> &str {
        match &self.access_token {
            Some(token) => token,
            None => "",
        }
    }
    fn new() -> Self {
        Program {
            session: Session::new(),
            access_token: None,
            // access_token: Some(String::from("Some Valid Access Token")),
            token: None,
        }
    }

    async fn call(&mut self, uri: &str) -> Result<String, SessionError> {
        info!("call() Begin");
        // self.access_token = Some(generate_token(self.session.to_owned()).await?);
        let res = check_token_valide(self.get_access_token()).await;
        if let Ok(false) = res {
            warn!("Token is not valid, please login again");
            return Err(SessionError::TokenNotValid);
        }
        let ac_token = self.access_token.as_ref().unwrap();
        let client_id = self.session.client_id.to_owned();
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", client_id.as_str()),
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
        info!("call() End");
        Ok(tmp)
    }
}

#[tokio::main]
async fn main() -> Result<(), SessionError> {
    info!("main() Begin");
    let config = Config::new()?;
    if config.list_commands {
        return list_available_commands();
    }
    let mut prog = Program::new();
    run(&mut prog, config).await?;
    Ok(())
}
