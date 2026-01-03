use crate::number_parsers;
use chrono::{Datelike, Local};
use serde::Serializer;
use serde::{Deserialize, Deserializer, Serialize, de::Error};
use std::fmt::{self, Display};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Error, Debug)]
pub enum DateError {
    #[error("date should have exactly 3 components")]
    InvalidSeparator,
    #[error("date has an invalid year")]
    InvalidYear,
    #[error("date has an invalid month")]
    InvalidMonth,
    #[error("date has an invalid day")]
    InvalidDay,
}

impl FromStr for Date {
    type Err = DateError;

    fn from_str(s: &str) -> Result<Date, DateError> {
        let mut segments = s.split('-');
        let year_portion = segments.next().ok_or(DateError::InvalidSeparator)?;
        let month_portion = segments.next().ok_or(DateError::InvalidSeparator)?;
        let day_portion = segments.next().ok_or(DateError::InvalidSeparator)?;
        if segments.next().is_some() {
            return Err(DateError::InvalidSeparator);
        }

        let year: u16 =
            number_parsers::unfixed_width(year_portion).ok_or(DateError::InvalidYear)?;
        let month: u8 =
            number_parsers::fixed_width(month_portion, 2).ok_or(DateError::InvalidMonth)?;
        if month > 12 {
            return Err(DateError::InvalidMonth);
        }
        let day: u8 = number_parsers::fixed_width(day_portion, 2).ok_or(DateError::InvalidDay)?;
        if day > 31 {
            return Err(DateError::InvalidDay);
        }

        return Ok(Date { year, month, day });
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Date::from_str(&s).map_err(D::Error::custom)
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        assert!(self.month > 0);
        assert!(self.month <= 12);
        assert!(self.day > 0);
        assert!(self.day <= 31);
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl Date {
    pub fn today() -> Date {
        let now = Local::now();

        let day: u32 = now.day();
        assert!(day <= 31);
        let month: u32 = now.month();
        assert!(month <= 12);
        let year: i32 = now.year();
        assert!(year >= 0);
        assert!(year <= u16::MAX.into());

        Date {
            year: year as u16,
            month: month as u8,
            day: day as u8,
        }
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    fn is_leap_year(&self) -> bool {
        if self.year % 400 == 0 {
            return true;
        }

        if self.year % 100 == 0 {
            return false;
        }

        return self.year % 4 == 0;
    }

    pub fn days_in_month(&self) -> u8 {
        assert!(self.month >= 1);
        assert!(self.month <= 12);

        match self.month {
            1 => 31,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            3 => 31,
            4 => 30,
            5 => 31,
            6 => 30,
            7 => 31,
            8 => 31,
            9 => 30,
            10 => 31,
            11 => 30,
            12 => 31,
            0 | 13..=u8::MAX => unreachable!(),
        }
    }

    fn short_month_name(&self) -> String {
        assert!(self.month >= 1);
        assert!(self.month <= 12);

        match self.month {
            1 => "Jan".to_string(),
            2 => "Feb".to_string(),
            3 => "Mar".to_string(),
            4 => "Apr".to_string(),
            5 => "May".to_string(),
            6 => "Jun".to_string(),
            7 => "Jul".to_string(),
            8 => "Aug".to_string(),
            9 => "Sep".to_string(),
            10 => "Oct".to_string(),
            11 => "Nov".to_string(),
            12 => "Dec".to_string(),
            0 | 13..=u8::MAX => unreachable!(),
        }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
