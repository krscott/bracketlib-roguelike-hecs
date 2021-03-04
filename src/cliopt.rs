use anyhow::{anyhow, Context};
use std::{convert::TryInto, fs, path::PathBuf};
use structopt::StructOpt;

use crate::config::{self, Config};

const DEFAULT_CONFIG_PATH: &'static str = ".rl-config";

#[derive(Debug, StructOpt)]
#[structopt(
    name = "bracketlib-book",
    about = "An implementation of the Bracketlib Roguelike Tutorial using hecs"
)]
struct Opt {
    /// Use an external config file
    #[structopt(
        short,
        long,
        parse(from_os_str),
        required_if("create-config", "true"),
        help = "Specify a config file to load"
    )]
    config: Option<PathBuf>,

    /// Create a default config file
    #[structopt(short = "z", long, help = "create a default config file")]
    create_config: bool,
}

pub struct ParsedOpt {
    pub config: Config,
}

pub fn parse_opt_args() -> anyhow::Result<ParsedOpt> {
    let opt = Opt::from_args();

    let user_config = if opt.create_config {
        let path = opt
            .config
            .unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_PATH));

        if path.exists() {
            return Err(anyhow!(format!(
                "Could not write config '{}'. File already exists.",
                path.to_string_lossy()
            )));
        }

        let user_config = config::default_user_config();
        let user_config_string = toml::to_string_pretty(&user_config)?;
        fs::write(path, user_config_string)?;

        user_config
    } else {
        match opt.config {
            Some(path) => {
                let config_string_string = fs::read_to_string(path)?;
                toml::from_str(&config_string_string).context("Error parsing config file")?
            }
            None => match fs::read_to_string(DEFAULT_CONFIG_PATH) {
                Ok(s) => toml::from_str(&s)?,
                Err(_) => config::default_user_config(),
            },
        }
    };

    let config = user_config.try_into()?;

    Ok(ParsedOpt { config })
}
