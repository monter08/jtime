use anyhow::{Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub jira_url: String,
    pub jira_token: String,
    #[serde(default)]
    pub show_weekends: bool,
}

impl Config {
    fn file_path() -> Result<String> {
        let home = std::env::var("HOME").context("Failed to get home directory")?;
        Ok(format!("{}/.jira-cli.json", home))
    }

    pub fn load() -> Result<Self> {
        let path = Self::file_path()?;
        if let Ok(file) = std::fs::File::open(&path) {
            let config: Config = serde_json::from_reader(file)?;
            return Ok(config);
        }

        let config = Config::prompt()?;
        config.save()?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::file_path()?;
        let file = std::fs::File::create(&path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn prompt() -> Result<Self> {
        println!(
            "{}\nEnter Jira URL:",
            "Hello! :) Let's set up your Jira CLI configuration.".green()
        );

        let mut jira_url = String::new();
        std::io::stdin().read_line(&mut jira_url)?;
        jira_url = jira_url.trim().to_string();

        println!("Enter Jira token (Get from: {}/secure/ViewProfile.jspa?selectedTab=com.atlassian.pats.pats-plugin:jira-user-personal-access-tokens):", &jira_url);
        let mut jira_token = String::new();
        std::io::stdin().read_line(&mut jira_token)?;
        jira_token = jira_token.trim().to_string();

        Ok(Config {
            jira_url,
            jira_token,
            show_weekends: false,
        })
    }
}
