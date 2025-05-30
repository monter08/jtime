use crate::api::{Jira, Nager};
use crate::cli::{Cli, Commands};
use crate::commands;
use crate::config::Config;
use anyhow::Result;

pub struct App {
    api: Jira,
    nager: Nager,
    config: Config,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        let api = Jira::new(config.jira_url.clone(), config.jira_token.clone());
        let nager = Nager::new(config.nager_url.clone(), config.nager_country_code.clone());
        Ok(Self { api, nager, config })
    }

    pub fn run(&self, cli: &Cli) -> Result<()> {
        match &cli.command {
            Commands::Log {
                task,
                time,
                day,
                yes,
            } => commands::log::execute(&self.api, &self.nager, task, time, day, yes),
            Commands::Month { cache, month } => {
                commands::month::execute(&self.config, &self.api, cache, month)
            }
            Commands::Week { cache, prev } => {
                commands::week::execute(&self.config, &self.api, prev, cache)
            }
            Commands::Config {
                url,
                token,
                nager_url,
                nager_country_code,
                show_weekends,
            } => commands::config::execute(
                self.config.clone(),
                url,
                token,
                nager_url,
                nager_country_code,
                show_weekends,
            ),
            Commands::Update => commands::update::execute(),
        }
    }
}
