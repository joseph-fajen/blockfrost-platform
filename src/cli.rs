use anyhow::{anyhow, Result};
use clap::{arg, command, Parser, ValueEnum};
use inquire::{
    validator::{ErrorMessage, Validation},
    Confirm, Select, Text,
};
use pallas_network::miniprotocols::{MAINNET_MAGIC, PREPROD_MAGIC, PREVIEW_MAGIC};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};
use std::fs;
use std::io::{self, Write};
use toml;
use tracing::Level;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    server_address: String,

    #[arg(long, default_value = "3000")]
    server_port: u16,

    #[arg(long, required_unless_present("init"))]
    network: Option<Network>,

    #[arg(long, default_value = "info")]
    log_level: LogLevel,

    #[arg(long, required_unless_present("init"))]
    node_socket_path: Option<String>,

    #[arg(long, default_value = "compact")]
    mode: Mode,

    /// Whether to run in solitary mode, without registering with the Icebreakers API
    #[arg(long)]
    solitary: bool,

    #[arg(long)]
    init: bool,

    #[arg(
        long,
        required_unless_present_any(&["solitary", "init"]),
        conflicts_with("solitary"),
        requires("reward_address")
    )]
    secret: Option<String>,

    #[arg(
        long,
        required_unless_present_any(&["solitary", "init"]),
        conflicts_with("solitary"),
        requires("secret")
    )]
    reward_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    server_address: String,
    server_port: u16,
    network: Network,
    log_level: LogLevel,
    node_socket_path: String,
    mode: Mode,
    secret: Option<String>,
    reward_address: Option<String>,
}

impl AppConfig {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: serde::de::DeserializeOwned,
    {
        let contents = fs::read_to_string(file_path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn to_file(&self, file_path: &str) -> Result<()>
    where
        Self: serde::Serialize,
    {
        let toml_string =
            toml::to_string(self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(toml_string.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum Mode {
    Compact,
    Light,
    Full,
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum Network {
    Mainnet,
    Preprod,
    Preview,
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Trace,
}

pub struct Config {
    pub server_address: String,
    pub server_port: u16,
    pub log_level: Level,
    pub network_magic: u64,
    pub node_socket_path: String,
    pub mode: Mode,
    pub icebreakers_config: Option<IcebreakersConfig>,
    pub max_pool_connections: usize,
    pub network: Network,
}

pub struct IcebreakersConfig {
    pub reward_address: String,
    pub secret: String,
}

fn enum_prompt<T: std::fmt::Debug>(message: &str, enum_values: &[T]) -> Result<String> {
    Select::new(
        message,
        enum_values
            .iter()
            .map(|it| format!("{:?}", it))
            .collect::<Vec<_>>(),
    )
    .prompt()
    .map_err(|e| anyhow!(e))
}

impl Config {
    pub fn init(args: Args) -> Result<Self> {
        if args.init {
            Self::generate_config()?;
        }
        let network = args.network.unwrap();
        let network_magic = Self::get_network_magic(&network);

        let icebreakers_config = match (args.solitary, args.reward_address, args.secret) {
            (false, Some(reward_address), Some(secret)) => Some(IcebreakersConfig {
                reward_address,
                secret,
            }),
            _ => None,
        };

        Ok(Config {
            server_address: args.server_address,
            server_port: args.server_port,
            log_level: args.log_level.into(),
            network_magic,
            node_socket_path: args.node_socket_path.unwrap(),
            mode: args.mode,
            icebreakers_config,
            max_pool_connections: 10,
            network,
        })
    }

    fn generate_config() -> Result<()> {
        let is_solitary = Confirm::new("Run in solitary mode?")
            .with_default(false)
            .with_help_message("Should be run without icebreakers API?")
            .prompt()?;

        let network = enum_prompt(
            "Which network are you connecting to?",
            Network::value_variants(),
        )
        .and_then(|it| Network::from_str(it.as_str(), true).map_err(|e| anyhow!(e)))?;

        let mode = enum_prompt("Mode?", Mode::value_variants())
            .and_then(|it| Mode::from_str(it.as_str(), true).map_err(|e| anyhow!(e)))?;

        let log_level = enum_prompt("What should be the log level?", LogLevel::value_variants())
            .and_then(|it| LogLevel::from_str(it.as_str(), true).map_err(|e| anyhow!(e)))?;

        let server_address = Text::new("Enter the server IP address:")
            .with_default("0.0.0.0")
            .with_validator(|input: &str| {
                input
                    .parse::<std::net::IpAddr>()
                    .map(|_| Validation::Valid)
                    .or_else(|_| {
                        Ok(Validation::Invalid(ErrorMessage::Custom(
                            "Invalid IP address".into(),
                        )))
                    })
            })
            .prompt()?;

        let server_port = Text::new("Enter the port number:")
            .with_default("3000")
            .with_validator(|input: &str| match input.parse::<u16>() {
                Ok(port) if port >= 1 => Ok(Validation::Valid),
                _ => Ok(Validation::Invalid(ErrorMessage::Custom(
                    "Invalid port number. It must be between 1 and 65535".into(),
                ))),
            })
            .prompt()
            .map_err(|e| anyhow!(e))
            .and_then(|it| it.parse::<u16>().map_err(|e| anyhow!(e)))?;

        let node_socket_path = Text::new("Enter patht to Cardano node socket:")
            .with_validator(|input: &str| {
                if input.is_empty() {
                    Ok(Validation::Invalid(ErrorMessage::Custom(
                        "Invalid path.".into(),
                    )))
                } else {
                    Ok(Validation::Valid)
                }
            })
            .prompt()?;

        let mut app_config = AppConfig {
            network,
            mode,
            log_level,
            server_address,
            server_port,
            node_socket_path,
            reward_address: None,
            secret: None,
        };

        if !is_solitary {
            let reward_address = Text::new("Enter the reward address:")
                .with_validator(|input: &str| {
                    if input.is_empty() {
                        Ok(Validation::Invalid(ErrorMessage::Custom(
                            "Invalid reward address.".into(),
                        )))
                    } else {
                        Ok(Validation::Valid)
                    }
                })
                .prompt()?;

            let secret = Text::new("Enter the icebreakers secret:")
                .with_validator(|input: &str| {
                    if input.is_empty() {
                        Ok(Validation::Invalid(ErrorMessage::Custom(
                            "Invalid reward address.".into(),
                        )))
                    } else {
                        Ok(Validation::Valid)
                    }
                })
                .prompt()?;
            app_config.reward_address = Some(reward_address);
            app_config.secret = Some(secret);
        }

        let file_path = std::env::current_dir().unwrap().join("blockfrost.toml");
        app_config.to_file(file_path.to_str().unwrap())?;
        println!("Config has been written to {:?}", file_path);

        std::process::exit(0);
    }

    fn get_network_magic(network: &Network) -> u64 {
        match network {
            Network::Mainnet => MAINNET_MAGIC,
            Network::Preprod => PREPROD_MAGIC,
            Network::Preview => PREVIEW_MAGIC,
        }
    }
}

// Implement conversion from LogLevel enum to tracing::Level
impl From<LogLevel> for Level {
    fn from(log_level: LogLevel) -> Self {
        match log_level {
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
            LogLevel::Trace => Level::TRACE,
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Mode::Compact => write!(f, "compact"),
            Mode::Light => write!(f, "light"),
            Mode::Full => write!(f, "full"),
        }
    }
}
