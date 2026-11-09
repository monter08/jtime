use crate::{
    api::{Jira, Nager},
    config::Config,
    view::{helper::Helper, Calendar, Render},
};
use anyhow::Result;
use chrono::{Datelike, Utc};
use colored::Colorize;

pub fn execute(
    config: &Config,
    api: &Jira,
    nager: &Nager,
    user: &str,
    month: &Option<u32>,
) -> Result<()> {
    let year = Utc::now().year();
    let month = month.unwrap_or_else(|| Utc::now().month());
    let range = Calendar::range_days_for_month(year, month)?;
    let tasks = api.fetch_worklogs_for(range.clone(), api.get_user_key(user)?)?;

    println!(
        "{}",
        format!("👀 Peeking at {}'s worklogs", user).yellow().bold()
    );
    println!(
        "{}",
        Calendar::render(
            range,
            tasks,
            config,
            Some(nager.get_all_holidays_map(year.to_string())?)
        )?
    );
    Ok(())
}
