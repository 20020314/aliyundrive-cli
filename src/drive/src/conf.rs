use crate::scan::model::AuthorizationToken;
use crate::scan::ClientType;
use crate::DateTime;
use anyhow::{anyhow, Context};
use lazy_static::lazy_static;
use serde::ser;
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

pub trait RW<T1, T2>
where
    T1: ser::Serialize,
    T2: ser::Serialize,
{
    fn print_std();

    fn write(t: T1) -> anyhow::Result<()>;

    fn read() -> anyhow::Result<T1>;

    fn read_token(is_mobile: bool) -> anyhow::Result<T2>;

    fn write_token(is_mobile: bool, t: T2) -> anyhow::Result<()>;
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    web_authorization: Option<Authorization>,
    app_authorization: Option<Authorization>,
}

impl Config {
    pub fn new(
        web_authorization: Option<Authorization>,
        app_authorization: Option<Authorization>,
    ) -> Self {
        Self {
            web_authorization,
            app_authorization,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Authorization {
    pub(crate) user_id: Option<String>,
    pub(crate) nick_name: Option<String>,
    pub(crate) client_type: Option<ClientType>,
    pub(crate) access_token: Option<String>,
    pub(crate) refresh_token: Option<String>,
    pub(crate) expire_time: Option<DateTime>,
}

impl AuthorizationToken for Authorization {
    fn access_token(&self) -> Option<String> {
        self.access_token.as_ref().cloned()
    }

    fn refresh_token(&self) -> Option<String> {
        self.refresh_token.as_ref().cloned()
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
            if let Some(token) = config.app_authorization {
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
            config.app_authorization = Some(t)
        } else {
            config.web_authorization = Some(t)
        }
        Conf::write(config)
    }
}
