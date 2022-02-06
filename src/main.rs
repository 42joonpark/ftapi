use ftapi::cli::{list_available_commands, Config};
use ftapi::error::SessionError;
use ftapi::session::Session;
use ftapi::structs::{me, token::TokenInfo};
use log::{self, info};

async fn run(prog: &mut Program, config: Config) -> Result<(), SessionError> {
    let command = config.command.to_owned();
    let cmd = command.trim().to_uppercase();
    match cmd.as_str() {
        "ID" => prog.id().await?,
        "ME" => prog.me().await?,
        "EMAIL" => prog.email().await?,
        "LOGIN" => prog.login().await?,
        "POINT" => prog.correction_point().await?,
        "WALLET" => prog.wallet().await?,
        _ => println!("Command `{}` not found", command),
    }
    Ok(())
}

struct Program {
    session: Session,
    // access_token: Option<String>,
    #[allow(dead_code)]
    token: Option<TokenInfo>,
}

impl Program {
    fn new() -> Result<Self, SessionError> {
        Ok(Program {
            session: Session::new()?,
            // access_token: None,
            token: None,
        })
    }

    async fn call(&mut self, uri: &str) -> Result<String, SessionError> {
        let res = self.session.call(uri).await?;
        Ok(res)
    }
}

impl Program {
    async fn get_me(&mut self) -> Result<me::Me, SessionError> {
        info!("get_me() Begin");
        let res = self.call("v2/me").await?;
        let me: me::Me = serde_json::from_str(res.as_str())?;
        info!("get_me() End");
        Ok(me)
    }

    pub async fn me(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
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

    pub async fn email(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Email", m.email);
        Ok(())
    }

    pub async fn wallet(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Wallet", m.wallet);
        Ok(())
    }

    pub async fn id(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "ID", m.id);
        Ok(())
    }

    pub async fn login(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Login", m.login);
        Ok(())
    }

    pub async fn correction_point(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Correction point", m.correction_point);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), SessionError> {
    env_logger::init();
    let config = Config::new()?;
    if config.list_commands {
        return list_available_commands();
    }
    let mut prog = Program::new()?;
    run(&mut prog, config).await?;
    Ok(())
}
