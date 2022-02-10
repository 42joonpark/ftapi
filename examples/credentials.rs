use ftapi::Session;
use ftapi::SessionError;
use ftapi::results::{me, user};
use ftapi::token::generate_token_credentials;
use ftapi::token::{AccessToken, TokenInfo};
use log::{info, debug, warn};
use url::Url;

struct Program {
    session: Session,
    // access_token: Option<String>,
    #[allow(dead_code)]
    token: Option<TokenInfo>,
}

impl Program {
    fn new() -> Result<Self, SessionError> {
        Ok(Program {
            session: Session::new("config.toml")?,
            // access_token: None,
            token: None,
        })
    }

    async fn call(&mut self, uri: &str) -> Result<String, SessionError> {
        let res = self.session.call_credentials(uri).await?;
        Ok(res)
    }

	async fn call_credentials(&mut self, url: &str) -> Result<String, SessionError> {
		let res = self.session.call_credentials(url).await?;
		Ok(res)
	}
}

impl Program {
    async fn get_user_with_login(&mut self) -> Result<user::UserElement, SessionError> {
		let url = "https://api.intra.42.fr/v2/users";
		let url = Url::parse_with_params(url, &[
			("client_id", self.session.get_client_id()),
			("filter[login]", self.session.get_login()),
		])?;

		println!("{:#?}", url);
        let res = self.call_credentials(url.as_str()).await?;
		println!("{:?}", res);
		let user: user::User = serde_json::from_str(res.as_str())?;
        Ok(user[0].clone())
    }

	async fn get_me(&mut self, id: i64) -> Result<me::Me, SessionError> {
		let url = format!("https://api.intra.42.fr/v2/users/{}", id);
		let url = Url::parse_with_params(&url, &[
			("client_id", self.session.get_client_id()),
		])?;

		let res = self.call_credentials(url.as_str()).await?;
		let me: me::Me = serde_json::from_str(res.as_str())?;
		Ok(me)
	}


	#[allow(dead_code)]
    pub async fn me(&mut self) -> Result<(), SessionError> {
        let tmp = self.get_user_with_login().await?;
		let id = tmp.id;
		let m = self.get_me(id).await?;
        let title = if m.titles.is_empty() {
            ""
        } else {
            m.titles[0].name.split(' ').next().unwrap_or("")
        };
        println!("{} | {} {}", m.displayname, title, m.login);
        println!("{:20}{}", "Wallet", m.wallet);
        println!("{:20}{}", "Evaluation points", m.correction_point);
        println!("{:20}{}", "Cursus", m.cursus_users[1].cursus.name);
        Ok(())
    }

	/*
	#[allow(dead_code)]
    pub async fn email(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Email", m.email);
        Ok(())
    }

	#[allow(dead_code)]
    pub async fn wallet(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Wallet", m.wallet);
        Ok(())
    }

	#[allow(dead_code)]
    pub async fn id(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "ID", m.id);
        Ok(())
    }

	#[allow(dead_code)]
    pub async fn login(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Login", m.login);
        Ok(())
    }

	#[allow(dead_code)]
    pub async fn correction_point(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Correction point", m.correction_point);
        Ok(())
    }
	*/
}

#[tokio::main]
async fn main () -> Result<(), SessionError> {
	let mut prog = Program::new()?;
	prog.session.generate_token().await?;
	prog.me().await?;
	Ok(())
}