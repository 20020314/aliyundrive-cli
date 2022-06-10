pub mod rw;

use anyhow::anyhow;
use lazy_static::lazy_static;
use rw::RW;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

lazy_static! {
    static ref WORK_DIR_PATH: Option<&'static PathBuf> = {
        let home_dir = option_env!("HOME").expect("user home directory does not exist");
        let p = std::path::PathBuf::from(home_dir).join(".aliyundrive");
        let x = Box::new(p);
        Some(Box::leak(x))
    };
    static ref CONFIG_FILE_PATH: Option<&'static PathBuf> = {
        let home_dir = option_env!("HOME").expect("user home directory does not exist");
        let p = std::path::PathBuf::from(home_dir)
            .join(".aliyundrive")
            .join("config.yaml");
        let x = Box::new(p);
        Some(Box::leak(x))
    };
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Config {
    web_authorization_token: Option<AuthorizationToken>,
    mobile_authorization_token: Option<AuthorizationToken>,
}

impl Config {
    pub fn new(p1: Option<AuthorizationToken>, p2: Option<AuthorizationToken>) -> Self {
        Self {
            web_authorization_token: p1,
            mobile_authorization_token: p2,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationToken {
    access_token: Option<String>,
    refresh_token: Option<String>,
}

impl AuthorizationToken {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token: Some(access_token),
            refresh_token: Some(refresh_token),
        }
    }
}

pub struct Context;

impl Context {
    pub fn init() -> anyhow::Result<()> {
        // work dir not exists
        let p = WORK_DIR_PATH.expect("Initialize aliyundrive directory");
        if !p.exists() {
            std::fs::create_dir(p)?;
            log::debug!("Initialize aliyundrive directory: {}", p.display())
        }

        // config file not exists
        let p = CONFIG_FILE_PATH.expect("Initialize config file error");
        if !p.exists() {
            let res = std::fs::File::create(p);
            match res {
                Ok(f) => {
                    serde_yaml::to_writer(f, &Config::default())?;
                    log::debug!("Initialize config file: {}", p.display());
                }
                Err(e) => {
                    return Err(anyhow!(e));
                }
            }
        }

        Ok(())
    }
}

impl RW<Config, AuthorizationToken> for Context {
    fn write(t: Config) -> serde_yaml::Result<()> {
        let p = CONFIG_FILE_PATH.expect("Initialize config file error");
        let f = std::fs::File::options()
            .write(true)
            .open(p)
            .expect("Failed to read configuration");
        serde_yaml::to_writer(f, &t)
    }

    fn read() -> serde_yaml::Result<Config> {
        let p = CONFIG_FILE_PATH.expect("Initialize aliyundrive directory");
        let f = std::fs::File::open(p).expect("Failed to read configuration");
        serde_yaml::from_reader::<std::fs::File, Config>(f)
    }

    fn read_token(is_mobile: bool) -> serde_yaml::Result<AuthorizationToken> {
        let config_result = Context::read();
        let config = match config_result {
            Ok(config) => config,
            Err(e) => return Err(serde_yaml::Error::from(e)),
        };
        if is_mobile {
            if let Some(token) = config.mobile_authorization_token {
                return Ok(token);
            }
        } else {
            if let Some(token) = config.web_authorization_token {
                return Ok(token);
            }
        }
        Ok(AuthorizationToken::default())
    }

    fn write_token(is_mobile: bool, t: AuthorizationToken) -> serde_yaml::Result<()> {
        let mut config = Context::read()?;
        if is_mobile {
            config.mobile_authorization_token = Some(t)
        } else {
            config.web_authorization_token = Some(t)
        }
        Context::write(config)
    }
}

#[cfg(test)]
mod tests {
    use crate::conf::rw::RW;
    use crate::conf::Context;
    use crate::conf::{AuthorizationToken, Config};

    #[test]
    fn read_write_test() {
        let read_config = Context::read().unwrap();
        println!("{:?}", read_config);

        let p1 = AuthorizationToken::new(String::from("a1"), String::from("a2"));
        let p2 = AuthorizationToken::new(String::from("a3"), String::from("a4"));
        let config = Config::new(Some(p1), Some(p2));
        Context::write(config).unwrap();
        let read_config = Context::read().unwrap();
        println!("{:?}", read_config);

        let t1 = Context::read_token(false).unwrap();
        println!("{:?}", t1);
        let t2 = Context::read_token(true).unwrap();
        println!("{:?}", t2);

        let p3 = AuthorizationToken::new(String::from("a5"), String::from("a6"));
        let p4 = AuthorizationToken::new(String::from("a7"), String::from("a8"));
        Context::write_token(false, p3).unwrap();
        Context::write_token(true, p4).unwrap();

        let read_config = Context::read().unwrap();
        println!("{:?}", read_config);
    }
}
