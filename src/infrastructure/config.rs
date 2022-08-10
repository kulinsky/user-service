use envconfig::Envconfig;

use crate::error::{Error, ErrorKind, Result};

#[derive(Envconfig)]
pub struct DbConfig {
    #[envconfig(from = "DB_HOST")]
    pub host: String,

    #[envconfig(from = "DB_PORT", default = "5432")]
    pub port: u16,

    #[envconfig(from = "DB_USER")]
    pub user: String,

    #[envconfig(from = "DB_PASSWORD")]
    pub password: String,

    #[envconfig(from = "DB_NAME")]
    pub name: String,

    #[envconfig(from = "DB_MAX_CONN", default = "5")]
    pub max_conn: u32,

    #[envconfig(from = "DB_POOL_SIZE", default = "5")]
    pub pool_size: u32,
}

impl DbConfig {
    pub fn get_connect_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(nested = true)]
    pub database: DbConfig,
}

impl Config {
    pub fn new_from_env() -> Result<Config> {
        Ok(Config::init_from_env()?)
    }
}

impl From<envconfig::Error> for Error {
    fn from(err: envconfig::Error) -> Self {
        Error {
            kind: ErrorKind::Configuration,
            message: err.to_string(),
        }
    }
}
