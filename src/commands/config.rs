use crate::config::Config;
use anyhow::Result;
use colored::Colorize;

pub fn execute(
    mut config: Config,
    url: &Option<String>,
    token: &Option<String>,
    nager_url: &Option<Option<String>>,
    nager_country_code: &Option<Option<String>>,
    show_weekends: &Option<bool>,
) -> Result<()> {
    if let Some(raw_url) = url {
        let clean_url = raw_url
            .trim_end_matches('/')
            .trim_start_matches("https://")
            .trim_start_matches("http://");
        config.jira_url = clean_url.to_string();
    }

    if let Some(token) = token {
        config.jira_token = token.clone();
    }
    if let Some(nager_url) = nager_url {
        config.nager_url = nager_url.clone();
    }
    if let Some(nager_country_code) = nager_country_code {
        config.nager_country_code = nager_country_code.clone();
    }
    if let Some(show_weekends) = show_weekends {
        config.show_weekends = *show_weekends;
    }

    if url.is_some()
        || token.is_some()
        || nager_url.is_some()
        || nager_country_code.is_some()
        || show_weekends.is_some()
    {
        config.save()?;
        println!("{}", "Configuration updated successfully! :)".green());
        return Ok(());
    }

    println!("Jira URL (url): {}", config.jira_url.green());
    println!("Jira token (token): {}", config.jira_token.green());
    if let Some(nager_url) = config.nager_url {
        println!("Nager URL (nager_url): {}", nager_url.green());
    }
    if let Some(nager_country_code) = config.nager_country_code {
        println!("Nager Country Code (nager_country_code): {}", nager_country_code.green());
    }
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
