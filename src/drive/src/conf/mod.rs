pub mod rw;

use crate::scan::ClientType;
use crate::DateTime;
use anyhow::{anyhow, Context};
use lazy_static::lazy_static;
use rw::RW;
use serde::{Deserialize, Serialize};
use std::io::Read;
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    web_authorization: Option<Authorization>,
    mobile_authorization: Option<Authorization>,
}

impl Config {
    pub fn new(
        web_authorization_token: Option<Authorization>,
        mobile_authorization_token: Option<Authorization>,
    ) -> Self {
        Self {
            web_authorization: web_authorization_token,
            mobile_authorization: mobile_authorization_token,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Authorization {
    client_type: Option<ClientType>,
    access_token: Option<String>,
    refresh_token: Option<String>,
    expire_time: Option<DateTime>,
}

impl Authorization {
    pub fn new(
        client_type: Option<ClientType>,
        access_token: Option<String>,
        refresh_token: Option<String>,
        expire_time: Option<DateTime>,
    ) -> Self {
        Self {
            client_type,
            access_token,
            refresh_token,
            expire_time,
        }
    }
}

pub struct Conf;

impl Conf {
    pub fn init() -> anyhow::Result<()> {
        // work dir not exists
        let p = WORK_DIR_PATH.expect("Initialize aliyundrive directory error");
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

impl RW<Config, Authorization> for Conf {
    fn print_std() {
        let p = CONFIG_FILE_PATH.expect("Initialize aliyundrive directory error");
        let mut f = std::fs::File::open(p).expect("Failed to read configuration");
        let mut config_str = String::new();
        f.read_to_string(&mut config_str)
            .expect("Read configuration error!");
        print!("{}", config_str)
    }

    fn write(t: Config) -> anyhow::Result<()> {
        let p = CONFIG_FILE_PATH.expect("Initialize config file error");
        let f = std::fs::File::options()
            .write(true)
            .open(p)
            .expect("Failed to read configuration");
        serde_yaml::to_writer(f, &t).context("Serialized write configuration failed")
    }

    fn read() -> anyhow::Result<Config> {
        let p = CONFIG_FILE_PATH.expect("Initialize aliyundrive directory error");
        let f = std::fs::File::open(p).expect("Failed to read configuration");
        serde_yaml::from_reader::<std::fs::File, Config>(f)
            .context("Serialized read configuration failed")
    }

    fn read_token(is_mobile: bool) -> anyhow::Result<Authorization> {
        let config = Conf::read().context("Read config error")?;
        if is_mobile {
            if let Some(token) = config.mobile_authorization {
                return Ok(token);
            }
        } else {
            if let Some(token) = config.web_authorization {
                return Ok(token);
            }
        }
        Ok(Authorization::default())
    }

    fn write_token(is_mobile: bool, t: Authorization) -> anyhow::Result<()> {
        let mut config = Conf::read()?;
        if is_mobile {
            config.mobile_authorization = Some(t)
        } else {
            config.web_authorization = Some(t)
        }
        Conf::write(config)
    }
}

#[cfg(test)]
mod tests {

    use crate::conf::rw::RW;
    use crate::conf::{Authorization, Conf, Config};

    #[test]
    fn read_write_test() {
        let read_config = Conf::read().unwrap();
        println!("{:?}", read_config);

        let p1 = Authorization::new(
            None,
            Some(String::from("a1")),
            Some(String::from("a2")),
            None,
        );
        let p2 = Authorization::new(
            None,
            Some(String::from("a3")),
            Some(String::from("a4")),
            None,
        );
        let config = Config::new(Some(p1), Some(p2));
        Conf::write(config).unwrap();
        let read_config = Conf::read().unwrap();
        println!("{:?}", read_config);

        let t1 = Conf::read_token(false).unwrap();
        println!("{:?}", t1);
        let t2 = Conf::read_token(true).unwrap();
        println!("{:?}", t2);

        let p3 = Authorization::new(
            None,
            Some(String::from("a5")),
            Some(String::from("a6")),
            None,
        );
        let p4 = Authorization::new(
            None,
            Some(String::from("a7")),
            Some(String::from("a8")),
            None,
        );
        Conf::write_token(false, p3).unwrap();
        Conf::write_token(true, p4).unwrap();

        let read_config = Conf::read().unwrap();
        println!("{:?}", read_config);
    }
}
