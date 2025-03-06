use super::Calendar;
use crate::models::DateRange;
use anyhow::{anyhow, Result};
use chrono::{Datelike, Duration, NaiveDate};

pub trait Helper {
    fn range_days_for_month(year: i32, month: u32) -> Result<DateRange>;
    fn range_days_for_week(date: NaiveDate) -> Result<DateRange>;
}

impl Helper for Calendar {
    fn range_days_for_month(year: i32, month: u32) -> Result<DateRange> {
        if month < 1 || month > 12 {
            return Err(anyhow!("Invalid month"));
        }

        let first_day =
            NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| anyhow!("Invalid date"))?;

        let last_day = if month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
                .ok_or_else(|| anyhow!("Invalid date"))?
                .pred_opt()
                .ok_or_else(|| anyhow!("Failed to calculate last day of month"))?
        } else {
            NaiveDate::from_ymd_opt(year, month + 1, 1)
                .ok_or_else(|| anyhow!("Invalid date"))?
                .pred_opt()
                .ok_or_else(|| anyhow!("Failed to calculate last day of month"))?
        };

        let monday = first_day
            .checked_sub_signed(Duration::days(
                first_day.weekday().num_days_from_monday() as i64
            ))
            .ok_or_else(|| anyhow!("Failed to calculate Monday date"))?;

        let last_sunday = last_day
            .checked_add_signed(Duration::days(
                7 - last_day.weekday().num_days_from_sunday() as i64,
            ))
            .ok_or_else(|| anyhow!("Failed to calculate Sunday date"))?;

        Ok(DateRange {
            from: monday,
            to: last_sunday,
        })
    }
    fn range_days_for_week(date: NaiveDate) -> Result<DateRange> {
        let monday = date
            .checked_sub_signed(Duration::days(date.weekday().num_days_from_monday() as i64))
            .ok_or_else(|| anyhow!("Failed to calculate Monday date"))?;

        let sunday = date
            .checked_add_signed(Duration::days(
                7 - date.weekday().num_days_from_sunday() as i64,
            ))
            .ok_or_else(|| anyhow!("Failed to calculate Sunday date"))?;

        Ok(DateRange {
            from: monday,
            to: sunday,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Helper;
    use super::*;

    #[test]
    fn test_range_days_for_month() {
        // Test for March 2025 (starts on Sunday)
        let result = Calendar::range_days_for_month(2025, 3).unwrap();
        assert_eq!(
            result,
            DateRange {
                from: NaiveDate::from_ymd_opt(2025, 02, 24).unwrap(),
                to: NaiveDate::from_ymd_opt(2025, 04, 06).unwrap(),
            }
        );
    }

    #[test]
    fn test_range_days_for_april_2025() {
        // Test for April 2025 (starts on Tuesday)
        let result = Calendar::range_days_for_month(2025, 4).unwrap();
        assert_eq!(
            result,
            DateRange {
                from: NaiveDate::from_ymd_opt(2025, 03, 31).unwrap(),
                to: NaiveDate::from_ymd_opt(2025, 05, 4).unwrap(),
            }
        );
    }

    #[test]
    fn test_range_days_for_week() {
        // Test for 2025-03-03 (Wednesday)
        let result =
            Calendar::range_days_for_week(NaiveDate::from_ymd_opt(2025, 3, 5).unwrap()).unwrap();
        assert_eq!(
            result,
            DateRange {
                from: NaiveDate::from_ymd_opt(2025, 3, 3).unwrap(),
                to: NaiveDate::from_ymd_opt(2025, 3, 9).unwrap(),
            }
        );
    }
}
