use crate::api::Jira;
use anyhow::{Context, Result};
use chrono::{Datelike, Duration, NaiveDate, Utc};
use colored::Colorize;

pub fn execute(api: &Jira, task: &str, time: &str, day: &str, yes: &bool) -> Result<()> {
    let time_spent = parse_time(time)?;
    let mut dates = parse_date(day, true)?;

    // Check if the date is a weekend
    let mut weekends_day = vec![];
    for date in dates.clone() {
        if date.weekday().num_days_from_monday() >= 5 {
            weekends_day.push(date);
        }
    }
    if !weekends_day.is_empty() {
        println!(
            "Hey! You're trying to log time on {}, which falls on the weekend. Do you want to log time? :) [y/N]",
            weekends_day
                .iter()
                .map(|d| d.format("%Y-%m-%d").to_string())
                .collect::<Vec<String>>()
                .join(", ")
                .green()
        );

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "y" => {
                // Keep all dates including weekends
            }
            "n" => {
                // Remove weekends from dates in-place
                dates.retain(|d| d.weekday().num_days_from_monday() < 5);
            }
            _ => {
                println!("Invalid input, please enter y or n");
                return Ok(());
            }
        }
    }

    for date in dates.clone() {
        println!(
            "Logging {} on {} for task {}",
            time.green(),
            date.format("%Y-%m-%d").to_string().green(),
            task.green()
        );
    }

    if !*yes {
        println!("Are you sure? [y/N]");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            println!("Aborted.");
            return Ok(());
        }
    }

    for date in dates {
        api.log_worktime(task, time_spent, &date).context(format!(
            "Failed to log time for {}",
            date.format("%Y-%m-%d").to_string()
        ))?;
    }

    println!(
        "{}",
        "Logged time successfully! Time for coffee! ☕".green()
    );

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
    fn test_parse_day_range_without_weeeknd() -> Result<()> {
        let parsed = parse_date("1-3", false)?;
        let now = Utc::now().naive_utc();
        assert_eq!(
            parsed,
            vec![
                NaiveDate::from_ymd_opt(now.year(), now.month(), 3).unwrap(), // 1-2 March 2025 is weekend
            ]
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
