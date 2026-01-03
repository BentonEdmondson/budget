use crate::date::Date;
use crate::{colors, number_parsers};
use serde::{Deserialize, Deserializer, de::Error};
use serde::{Serialize, Serializer};
use std::fmt;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, PartialOrd, Ord, Copy)]
pub struct Money {
    cents: i64,
}

#[derive(Debug, PartialEq)]
pub enum MoneyError {
    InvalidDecimalPoint,
    InvalidDollars,
    InvalidCents,
    Overflow,
}

impl AddAssign for Money {
    fn add_assign(&mut self, other: Money) {
        self.cents += other.cents;
    }
}

impl Money {
    pub fn from_cents(cents: i64) -> Money {
        Money { cents }
    }

    pub fn checked_div(&self, other: Money) -> Option<i64> {
        self.cents.checked_div(other.cents)
    }

    pub fn left_to_date_in_month(date: Date, limit: Money, spent: Money) -> Option<Money> {
        Some(Money {
            cents: {
                let cents_per_day = limit.cents.checked_div(date.days_in_month().into())?;
                let cents_accumulated = cents_per_day.checked_mul(date.day().into())?;
                let net_available = cents_accumulated.checked_sub(spent.cents)?;
                net_available
            },
        })
    }
}

impl FromStr for Money {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Money, MoneyError> {
        let mut segments = s.split('.');
        let dollars_portion = segments.next().ok_or(MoneyError::InvalidDecimalPoint)?;
        let cents_portion = segments.next().ok_or(MoneyError::InvalidDecimalPoint)?;
        if segments.next().is_some() {
            return Err(MoneyError::InvalidDecimalPoint);
        }

        let dollars: i64 =
            number_parsers::unfixed_width(dollars_portion).ok_or(MoneyError::InvalidDollars)?;
        let cents: i64 =
            number_parsers::fixed_width(cents_portion, 2).ok_or(MoneyError::InvalidCents)?;

        assert!(cents < 100);

        let dollars_times_100 = dollars.checked_mul(100).ok_or(MoneyError::Overflow)?;
        let total_cents = cents
            .checked_add(dollars_times_100)
            .ok_or(MoneyError::Overflow)?;

        return Ok(Money { cents: total_cents });
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.cents >= 0 {
            write!(
                f,
                "{}${}.{:02}{}",
                colors::GREEN,
                self.cents / 100,
                self.cents % 100,
                colors::RESET
            )
        } else {
            write!(
                f,
                "{}−${}.{:02}{}",
                colors::RED,
                -self.cents / 100,
                -self.cents % 100,
                colors::RESET
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Money::from_str("1.15").unwrap(), Money::from_cents(115));
    }

    #[test]
    fn test_large() {
        assert_eq!(
            Money::from_str("92233720368547758.07").unwrap(),
            Money::from_cents(i64::MAX)
        );
    }

    #[test]
    fn test_invalid_dollars() {
        assert_eq!(
            Money::from_str("©1.00").unwrap_err(),
            MoneyError::InvalidDollars
        );
        assert_eq!(
            Money::from_str(".15").unwrap_err(),
            MoneyError::InvalidDollars
        );
        assert_eq!(
            Money::from_str("-1.15").unwrap_err(),
            MoneyError::InvalidDollars
        );
        assert_eq!(
            Money::from_str("ab.15").unwrap_err(),
            MoneyError::InvalidDollars
        );
        assert_eq!(
            Money::from_str("1,500.15").unwrap_err(),
            MoneyError::InvalidDollars
        );
    }

    #[test]
    fn test_invalid_decimal_point() {
        assert_eq!(
            Money::from_str("1500").unwrap_err(),
            MoneyError::InvalidDecimalPoint
        );
        assert_eq!(
            Money::from_str("15.0.0").unwrap_err(),
            MoneyError::InvalidDecimalPoint
        );
    }

    #[test]
    fn test_invalid_cents() {
        assert_eq!(
            Money::from_str("15.ab").unwrap_err(),
            MoneyError::InvalidCents
        );
        assert_eq!(
            Money::from_str("15.-15").unwrap_err(),
            MoneyError::InvalidCents
        );
        assert_eq!(
            Money::from_str("15.101").unwrap_err(),
            MoneyError::InvalidCents
        );
        assert_eq!(
            Money::from_str("15.1").unwrap_err(),
            MoneyError::InvalidCents
        );
        assert_eq!(
            Money::from_str("151.").unwrap_err(),
            MoneyError::InvalidCents
        );
    }

    #[test]
    fn test_overflow() {
        assert_eq!(
            Money::from_str("92233720368547758.08").unwrap_err(),
            MoneyError::Overflow
        );
    }

    #[test]
    fn test_pluses() {
        assert_eq!(
            Money::from_str("+100.22").unwrap_err(),
            MoneyError::InvalidDollars
        );
        assert_eq!(
            Money::from_str("100.+2").unwrap_err(),
            MoneyError::InvalidCents
        );
    }
}

impl fmt::Display for MoneyError {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Trying to serialize a MoneyError; it needs to look nice!");
    }
}

impl<'de> Deserialize<'de> for Money {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Money::from_str(&s).map_err(D::Error::custom)
    }
}

impl Serialize for Money {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let cents: i64 = self.cents % 100;
        let dollars: i64 = self.cents / 100;
        serializer
            .serialize_str(format!("{}.{:02}", dollars.to_string(), cents.to_string()).as_str())
    }
}
