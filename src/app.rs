use crate::api::Jira;
use crate::cli::{Cli, Commands};
use crate::commands;
use crate::config::Config;
use anyhow::Result;

pub struct App {
    api: Jira,
    config: Config,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        let api = Jira::new(config.jira_url.clone(), config.jira_token.clone());
        Ok(Self { api, config })
    }

    pub fn run(&self, cli: &Cli) -> Result<()> {
        match &cli.command {
            Commands::Log {
                task,
                time,
                day,
                yes,
            } => commands::add::execute(&self.api, task, time, day, yes),
            Commands::Month { cache, month } => {
                commands::month::execute(&self.config, &self.api, cache, month)
            }
            Commands::Week { cache, prev } => {
                commands::week::execute(&self.config, &self.api, prev, cache)
            }
            Commands::Config {
                url,
                token,
                show_weekends,
            } => commands::config::execute(self.config.clone(), url, token, show_weekends),
        }
    }
}
