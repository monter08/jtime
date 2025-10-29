use crate::api::nager::NagerHoliday;
use crate::api::{Jira, Nager};
use anyhow::{Context, Result};
use chrono::{Datelike, Duration, NaiveDate, Utc};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

pub fn execute(
    api: &Jira,
    nager: &Nager,
    cli_task: &Option<String>,
    time: &str,
    cli_day: &Option<String>,
    cli_comment: &Option<String>,
    yes: &bool,
) -> Result<()> {
    let task = match cli_task {
        Some(t) => t,
        None => {
            let actually_works = api.actually_works()?;
            if actually_works.is_empty() {
                anyhow::bail!("No tasks found. Please set up your tasks first.");
            }

            let days: Vec<String> = actually_works
                .iter()
                .map(|d| format!("({}) {}", d.id, d.name))
                .chain(std::iter::once("Cancel operation".to_string()))
                .collect();

            let day = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Please select the task you want to log time for:")
                .items(&days)
                .default(0)
                .interact()?;
            if day == days.len() - 1 {
                println!("Aborted.");
                return Ok(());
            }
            &actually_works[day].id.clone()
        }
    };

    let day = match cli_day.clone() {
        Some(d) => d,
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Please enter the day(s) you want to log time for (e.g., 2 or 2-5):")
            .default("today".to_string())
            .interact_text()?,
    };

    let time_spent = parse_time(time)?;
    let mut dates = parse_date(day.as_str(), true)?;

    match check_weekends(&mut dates) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
    };

    if dates.is_empty() {
        println!("Nothing to log");
        return Ok(());
    }

    match check_holidays(nager, &mut dates) {
        Ok(_) => (),
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
    };

    for date in dates.clone() {
        println!(
            "Logging {} on {} for task {}",
            time.green(),
            date.format("%Y-%m-%d").to_string().green(),
            task.green()
        );
    }

    let comment = match cli_comment.clone() {
        Some(c) => Some(c),
        None => {
            if task.contains("TD-") {
                Some(
                    Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Please enter comment for TD task (e.g., retro):")
                        .default("review, planning".to_string())
                        .interact_text()?,
                )
            } else {
                None
            }
        }
    };

    if !*yes
        && !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Are you sure?")
            .default(true)
            .show_default(true)
            .wait_for_newline(true)
            .interact()?
    {
        println!("Aborted.");
        return Ok(());
    }

    for date in dates {
        api.log_worktime(task, time_spent, &date, comment.clone())
            .context(format!(
                "Failed to log time for {}",
                date.format("%Y-%m-%d")
            ))?;
    }

    println!(
        "{}",
        "Logged time successfully! Time for coffee! ☕".green()
    );

    Ok(())
}

fn check_weekends(dates: &mut Vec<NaiveDate>) -> Result<()> {
    if dates.is_empty() {
        return Ok(());
    }

    // Check if the date is a weekend
    let weekends_day: Vec<NaiveDate> = dates
        .iter()
        .filter(|d| d.weekday().num_days_from_monday() >= 5)
        .cloned()
        .collect();

    if !weekends_day.is_empty() {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Hey! You're trying to log time on {}, which falls on the weekend. Do you want to log time? :)",
                weekends_day
                    .iter()
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
                    .green()
            ))
            .default(0)
            .items(&["Skip weekend days", "Keep weekend days", "Cancel operation"])
            .interact()?;

        match selection {
            0 => dates.retain(|d| d.weekday().num_days_from_monday() < 5),
            1 => {} // Keep all dates
            _ => {
                anyhow::bail!("Aborted.")
            }
        };
    }

    Ok(())
}

fn check_holidays(nager: &Nager, dates: &mut Vec<NaiveDate>) -> Result<()> {
    if dates.is_empty() {
        return Ok(());
    }

    let first_date = dates.first().expect("No date found");
    let holidays = match nager.get_all_holidays(first_date.format("%Y").to_string()) {
        Ok(h) => h,
        Err(err) => {
            println!("{}", err);
            if !Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Can't check holidays. Do you want to continue without it?")
                .default(true)
                .show_default(true)
                .wait_for_newline(true)
                .interact()?
            {
                anyhow::bail!("Aborted.");
            } else {
                return Ok(());
            }
        }
    };

    let holiday_dates: Vec<&NagerHoliday> = dates
        .iter()
        .filter_map(|d| {
            holidays
                .iter()
                .find(|h| h.date == *d.format("%Y-%m-%d").to_string())
        })
        .collect();

    if !holiday_dates.is_empty() {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Hey! You're trying to log time on holiday(s):\n{}\n\nWhat do you want to do? :)",
                holiday_dates
                    .iter()
                    .map(|d| format!("{} - {}", d.date, d.local_name.clone()))
                    .collect::<Vec<_>>()
                    .join("\n")
                    .green()
            ))
            .default(0)
            .items(&["Skip holiday days", "Keep holiday days", "Cancel operation"])
            .interact()?;

        match selection {
            0 => dates.retain(|d| {
                holiday_dates
                    .iter()
                    .all(|h| h.date != d.format("%Y-%m-%d").to_string())
            }),
            1 => {} // Keep all dates
            _ => {
                anyhow::bail!("Aborted.")
            }
        };
    }

    Ok(())
}

pub fn parse_date(date_str: &str, with_weekend: bool) -> Result<Vec<NaiveDate>> {
    let now = Utc::now().naive_utc().date();
    let (year, month) = (now.year(), now.month());

    let parse_single_date = |s: &str| -> Result<NaiveDate> {
        if let Ok(day) = s.parse::<u32>() {
            NaiveDate::from_ymd_opt(year, month, day)
                .ok_or_else(|| anyhow::anyhow!("Invalid day {} for the current month", day))
        } else {
            match s {
                "today" => Ok(now),
                "yesterday" => Ok(now - Duration::days(1)),
                _ => Ok(NaiveDate::parse_from_str(s, "%d-%m-%Y")
                    .map_err(|_| anyhow::anyhow!("Invalid date format: {}", s))?),
            }
        }
    };

    if let Some(idx) = date_str.find('-') {
        let (start_str, end_str) = date_str.split_at(idx);
        let end_str = &end_str[1..];
        if let (Ok(start_day), Ok(end_day)) = (
            start_str.trim().parse::<u32>(),
            end_str.trim().parse::<u32>(),
        ) {
            let dates = (start_day..=end_day)
                .filter_map(|d| NaiveDate::from_ymd_opt(year, month, d))
                .filter(|d| with_weekend || d.weekday().num_days_from_monday() < 5)
                .collect::<Vec<_>>();
            if dates.is_empty() {
                anyhow::bail!("No valid dates in the specified range");
            }
            Ok(dates)
        } else {
            let start_date = parse_single_date(start_str.trim())?;
            let end_date = parse_single_date(end_str.trim())?;
            Ok(vec![start_date, end_date])
        }
    } else {
        Ok(vec![parse_single_date(date_str)?])
    }
}

pub fn parse_time(time_str: &str) -> anyhow::Result<u64> {
    let mut seconds = 0;
    let mut current_num = String::new();

    for c in time_str.chars() {
        if c.is_ascii_digit() {
            current_num.push(c);
        } else {
            if current_num.is_empty() {
                return Err(anyhow::anyhow!("No number provided for time unit '{}'", c));
            }
            let num: u64 = current_num
                .parse()
                .map_err(|e| anyhow::anyhow!("Failed to parse number: {}", e))?;
            match c {
                'h' => seconds += num * 3600,
                'm' => seconds += num * 60,
                _ => return Err(anyhow::anyhow!("Invalid time unit: '{}'", c)),
            }
            current_num.clear();
        }
    }

    if !current_num.is_empty() {
        return Err(anyhow::anyhow!(
            "Incomplete time format, missing unit for '{}'",
            current_num
        ));
    }

    Ok(seconds)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Utc};

    #[test]
    fn test_parse_today() -> Result<()> {
        let parsed = parse_date("today", false)?;
        assert_eq!(parsed.len(), 1);
        let now = Utc::now().naive_utc().date();
        assert_eq!(parsed[0], now);
        Ok(())
    }

    #[test]
    fn test_parse_yesterday() -> Result<()> {
        let parsed = parse_date("yesterday", false)?;
        assert_eq!(parsed.len(), 1);
        let now = Utc::now().naive_utc().date();
        assert_eq!(parsed[0], now - chrono::Duration::days(1));
        Ok(())
    }

    #[test]
    fn test_parse_single_day() -> Result<()> {
        let parsed = parse_date("5", true)?;
        assert_eq!(parsed.len(), 1);
        Ok(())
    }

    #[test]
    fn test_parse_day_range() -> Result<()> {
        let parsed = parse_date("1-3", true)?;
        let now = Utc::now().naive_utc();
        assert_eq!(
            parsed,
            vec![
                NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap(),
                NaiveDate::from_ymd_opt(now.year(), now.month(), 2).unwrap(),
                NaiveDate::from_ymd_opt(now.year(), now.month(), 3).unwrap(),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_parse_day_range_without_weekend() -> Result<()> {
        // Find a range in the current month where the first two days are weekend and the third is a weekday
        // For robustness, search for such a range in the current month
        let now = Utc::now().naive_utc().date();
        let (year, month) = (now.year(), now.month());
        let mut found = false;
        for start_day in 1..=28 {
            let d1 = NaiveDate::from_ymd_opt(year, month, start_day);
            let d2 = NaiveDate::from_ymd_opt(year, month, start_day + 1);
            let d3 = NaiveDate::from_ymd_opt(year, month, start_day + 2);
            if let (Some(d1), Some(d2), Some(d3)) = (d1, d2, d3) {
                if d1.weekday().num_days_from_monday() >= 5
                    && d2.weekday().num_days_from_monday() >= 5
                    && d3.weekday().num_days_from_monday() < 5
                {
                    let range = format!("{}-{}", start_day, start_day + 2);
                    let parsed = parse_date(&range, false)?;
                    assert_eq!(parsed, vec![d3]);
                    found = true;
                    break;
                }
            }
        }
        assert!(
            found,
            "Could not find a suitable weekend-to-weekday range in this month"
        );
        Ok(())
    }

    #[test]
    fn test_parse_time_hours_only() -> Result<()> {
        // 2 hours = 7200 seconds
        let seconds = parse_time("2h")?;
        assert_eq!(seconds, 7200);
        Ok(())
    }

    #[test]
    fn test_parse_time_minutes_only() -> Result<()> {
        // 45 minutes = 2700 seconds
        let seconds = parse_time("45m")?;
        assert_eq!(seconds, 2700);
        Ok(())
    }

    #[test]
    fn test_parse_time_hour_and_minutes() -> Result<()> {
        // 1 hour and 30 minutes = 3600 + 1800 = 5400 seconds
        let seconds = parse_time("1h30m")?;
        assert_eq!(seconds, 5400);
        Ok(())
    }

    #[test]
    fn test_parse_time_invalid_unit() {
        // Using an invalid unit 's'
        let result = parse_time("10s");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_time_incomplete_format() {
        // Trailing number without unit
        let result = parse_time("15");
        assert!(result.is_err());
    }
}
