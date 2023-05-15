use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

use clap::{Args, Parser};
use figment::{
    providers::{Env, Format, Serialized, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};

// Command line parsing options. Serialize/Deserialize allows for using figment's Serialized
// provider
// skip_serializing_if option means if the value is not set on the command line it will not be set
// in the serialized structure
#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct Cli {
    #[arg(long = "cfg", value_name = "FILE")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub cfg_path: Option<std::path::PathBuf>,

    #[arg(long = "host")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub host: Option<IpAddr>,

    #[arg(long = "port")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub port: Option<u16>,

    #[arg(long = "log_level", value_name = "error|warn|info|debug|trace")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub log_level: Option<String>,

    #[clap(flatten)]
    pub db: CliDb,
}

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct CliDb {
    #[arg(name = "db_username", long = "db_username", value_name = "STRING")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub username: Option<String>,

    #[arg(name = "db_password", long = "db_password", value_name = "STRING")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub password: Option<String>,

    #[arg(name = "db_port", long = "db_port", value_name = "u16")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub port: Option<u16>,

    #[arg(name = "db_host", long = "db_host", value_name = "STRING")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub host: Option<String>,

    #[arg(name = "db_name", long = "db_name", value_name = "STRING")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: Option<String>,
}

// Application configuration
// serde(default) uses type's implementation of std::default::Default
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    // host
    pub host: IpAddr,

    // port
    pub port: u16,

    // logging level
    pub log_level: String,

    // database configuration
    pub db: DbConfig,
}

// Sets default values for config attributes
impl Default for Config {
    fn default() -> Self {
        Config {
            host: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            port: 8000,
            log_level: "info".to_string(),
            db: DbConfig::default(),
        }
    }
}

impl Config {
    // Validates if all required attributes are provided
    fn validate(&self) -> anyhow::Result<()> {
        // if self.host.is_unspecified() {
        // anyhow::bail!("missing host")
        // }
        if self.port == 0 {
            anyhow::bail!("missing port")
        }
        if self.log_level.is_empty() {
            anyhow::bail!("missing log level")
        }

        self.db.validate()?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct DbConfig {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub name: String,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            username: "postgres".to_string(),
            password: "".to_string(),
            port: 5432,
            host: "".to_string(),
            name: "newsletter".to_string(),
        }
    }
}

impl DbConfig {
    fn validate(&self) -> anyhow::Result<()> {
        if self.username.is_empty() {
            anyhow::bail!("missing database username")
        }
        if self.password.is_empty() {
            anyhow::bail!("missing database password")
        }
        if self.port == 0 {
            anyhow::bail!("missing database port")
        }
        if self.host.is_empty() {
            anyhow::bail!("missing database host")
        }
        if self.name.is_empty() {
            anyhow::bail!("missing database name")
        }

        Ok(())
    }

    // Constructs Postgres database connection string
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        )
    }

    // Constructs a connection to the server not a dedicated database
    pub fn server_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn load() -> anyhow::Result<Config> {
    let cli = Cli::parse();
    let cfg_path = match cli.cfg_path.clone() {
        Some(p) => p,
        None => std::path::PathBuf::from("config.yaml"),
    };

    let cfg: Config = Figment::new()
        .merge(Yaml::file(cfg_path))
        .merge(Env::prefixed("APP_"))
        .merge(Serialized::defaults(cli))
        .extract()?;

    // validate config
    cfg.validate()?;

    // initialize logger
    env_logger::builder()
        .filter_level(log::LevelFilter::from_str(&cfg.log_level)?)
        .format_target(false)
        .init();

    Ok(cfg)
}
