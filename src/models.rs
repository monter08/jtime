use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, PartialEq, Clone)]
pub struct DateRange {
    pub from: NaiveDate,
    pub to: NaiveDate,
}

impl DateRange {
    pub fn into_iter(self) -> DateRangeIter {
        DateRangeIter {
            current: self.from,
            end: self.to,
        }
    }

    pub fn days(self, with_weekends: bool) -> Vec<NaiveDate> {
        let days: Vec<_> = self.into_iter().collect();
        if with_weekends {
            days
        } else {
            days.into_iter()
                .filter(|date| {
                    !chrono::Datelike::weekday(date).eq(&chrono::Weekday::Sat)
                        && !chrono::Datelike::weekday(date).eq(&chrono::Weekday::Sun)
                })
                .collect()
        }
    }
}

pub struct DateRangeIter {
    current: NaiveDate,
    end: NaiveDate,
}

impl Iterator for DateRangeIter {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            None
        } else {
            let date = self.current;
            self.current += chrono::Duration::days(1);
            Some(date)
        }
    }
}

impl IntoIterator for DateRange {
    type Item = NaiveDate;
    type IntoIter = DateRangeIter;

    fn into_iter(self) -> Self::IntoIter {
        DateRangeIter {
            current: self.from,
            end: self.to,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Task {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct WorkLog {
    pub day: DateTime<Utc>,
    pub task: String,
    pub time_spent: String,
}

pub type WorkLogList = Vec<WorkLog>;
pub trait WorkLogListExt {
    fn get_by_day(&self, day: NaiveDate) -> WorkLogList;
}

impl WorkLogListExt for WorkLogList {
    fn get_by_day(&self, day: NaiveDate) -> WorkLogList {
        self.iter()
            .filter(|worklog| worklog.day.naive_utc().date() == day)
            .cloned()
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_range_iterator() {
        let from = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2023, 1, 5).unwrap();
        let range = DateRange { from, to };

        let dates: Vec<NaiveDate> = range.into_iter().collect();

        assert_eq!(dates.len(), 5);
        assert_eq!(dates[0], from);
        assert_eq!(dates[4], to);
    }

    #[test]
    fn test_date_range_days_with_weekends() {
        // 2023-1-7 is Saturday and 2023-1-8 is Sunday
        let from = NaiveDate::from_ymd_opt(2023, 1, 6).unwrap(); // Friday
        let to = NaiveDate::from_ymd_opt(2023, 1, 9).unwrap(); // Monday
        let range = DateRange { from, to };

        let days_with_weekends = range.clone().days(true);
        assert_eq!(days_with_weekends.len(), 4);

        let days_without_weekends = range.days(false);
        assert_eq!(days_without_weekends.len(), 2);
        assert_eq!(days_without_weekends[0], from);
        assert_eq!(
            days_without_weekends[1],
            NaiveDate::from_ymd_opt(2023, 1, 9).unwrap()
        );
    }

    #[test]
    fn test_worklog_get_by_day() {
        let day1 = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let day2 = NaiveDate::from_ymd_opt(2023, 1, 2).unwrap();

        let log1 = WorkLog {
            day: DateTime::<Utc>::from_naive_utc_and_offset(
                day1.and_hms_opt(10, 0, 0).unwrap(),
                Utc,
            ),
            task: "Task1".to_string(),
            time_spent: "2h".to_string(),
        };

        let log2 = WorkLog {
            day: DateTime::<Utc>::from_naive_utc_and_offset(
                day1.and_hms_opt(14, 0, 0).unwrap(),
                Utc,
            ),
            task: "Task2".to_string(),
            time_spent: "3h".to_string(),
        };

        let log3 = WorkLog {
            day: DateTime::<Utc>::from_naive_utc_and_offset(
                day2.and_hms_opt(9, 0, 0).unwrap(),
                Utc,
            ),
            task: "Task3".to_string(),
            time_spent: "4h".to_string(),
        };

        let logs = vec![log1, log2, log3];

        let day1_logs = logs.get_by_day(day1);
        assert_eq!(day1_logs.len(), 2);
        assert_eq!(day1_logs[0].task, "Task1");
        assert_eq!(day1_logs[1].task, "Task2");

        let day2_logs = logs.get_by_day(day2);
        assert_eq!(day2_logs.len(), 1);
        assert_eq!(day2_logs[0].task, "Task3");

        let day3_logs = logs.get_by_day(NaiveDate::from_ymd_opt(2023, 1, 3).unwrap());
        assert_eq!(day3_logs.len(), 0);
    }
}
