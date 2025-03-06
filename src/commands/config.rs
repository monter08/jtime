use crate::config::Config;
use anyhow::Result;
use colored::Colorize;

pub fn execute(
    mut config: Config,
    url: &Option<String>,
    token: &Option<String>,
    show_weekends: &Option<bool>,
) -> Result<()> {
    if let Some(url) = url {
        config.jira_url = url.clone();
    }
    if let Some(token) = token {
        config.jira_token = token.clone();
    }
    if let Some(show_weekends) = show_weekends {
        config.show_weekends = *show_weekends;
    }

    if url.is_some() || token.is_some() || show_weekends.is_some() {
        config.save()?;
    }

    println!("Jira URL (url): {}", config.jira_url.green());
    println!("Jira token (token): {}", config.jira_token.green());
    println!(
        "Show weekends (show_weekends): {}",
        config.show_weekends.to_string().green()
    );
    println!(
        "{} {}",
        "You can change the values:".yellow(),
        "jtime config --url https://jira.com --token 123 --show-weekends true".blue()
    );

    Ok(())
}
