use crate::cache::Cache;
use anyhow::{Context, Result};
use colored::Colorize;
use reqwest::blocking::Client;
use serde_json::Value;
use std::process::Command;
use std::time::Duration;

pub fn get_latest_version() -> Result<String> {
    let cache = Cache::new("latest-version".to_string());
    if let Some(cached) = cache.load().unwrap_or(None) {
        return Ok(cached);
    }
    let client = Client::new();
    let resp = client
        .get("https://api.github.com/repos/monter08/jtime/releases/latest")
        .header("User-Agent", "jtime-update-check")
        .timeout(Duration::from_secs(10))
        .send()
        .context("Failed to connect to GitHub API")?;

    let body: Value = resp.json().context("Failed to parse GitHub API response")?;
    let tag_name = body["tag_name"]
        .as_str()
        .context("Could not find tag_name in GitHub API response")?
        .trim_start_matches('v')
        .trim_end_matches('\n')
        .to_string();

    cache.save(&tag_name)?;
    Ok(tag_name)
}

pub fn current_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn execute() -> Result<()> {
    println!("Checking for updates...");

    let current_version = current_version();
    println!("Current version: {}", current_version.blue());

    let latest_version = get_latest_version()?;
    println!("Latest version: {}", latest_version.blue());

    if latest_version != current_version {
        println!("{} A new version is available!", "✓".green());
        println!(
            "Updating from {} to {}...",
            current_version.blue(),
            latest_version.blue()
        );

        // Run the install script
        let install_cmd =
            "curl -sSL https://raw.githubusercontent.com/monter08/jtime/main/install.sh | bash";

        println!("Running update script...");
        let status = Command::new("sh")
            .arg("-c")
            .arg(install_cmd)
            .status()
            .context("Failed to execute install script")?;

        if status.success() {
            println!(
                "{} JTime has been updated successfully to version {}!",
                "✓".green(),
                latest_version.green()
            );
        } else {
            println!(
                "{} Update failed. Please try again or install manually.",
                "✗".red()
            );
        }
    } else {
        println!("{} You're already running the latest version!", "✓".green());
    }

    Ok(())
}
