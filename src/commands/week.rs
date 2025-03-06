use crate::{
    api::API,
    cache::Cache,
    config::Config,
    view::{helper::Helper, Calendar, Render},
};
use anyhow::Result;

use chrono::Utc;

pub fn execute(config: &Config, api: &API, prev: &bool, use_cache: &bool) -> Result<()> {
    let cache = Cache::new(if *prev { "previous" } else { "current" }.to_string());

    if *use_cache {
        if let Some(data) = cache.load() {
            println!("{}", data);
            return Ok(());
        }
    }

    // Prev week
    let today = Utc::now().date_naive();
    let date = if *prev {
        today - chrono::Duration::days(7)
    } else {
        today
    };

    let range = Calendar::range_days_for_week(date)?;
    let tasks = api.fetch_worklogs(range.clone())?;

    let actually_works = api.actually_works()?;
    let output = format!(
        "{}{}",
        Calendar::render(range, tasks, config.show_weekends)?,
        Calendar::works_on(actually_works)
    );

    println!("{}", output);
    cache.save(&output);

    Ok(())
}
