use std::str::FromStr;

// TODO: I would imagine this is not idiomatic
pub trait Number {}
impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for u128 {}
impl Number for usize {}
impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}
impl Number for isize {}

// leading zeros are allowed for this function
// width must be at least 1
pub fn fixed_width<T: Number + FromStr>(s: &str, width: usize) -> Option<T> {
    assert!(width >= 1);

    if !has_only_numbers(s) {
        return None;
    }

    if s.len() != width {
        return None;
    }

    return s.parse().ok();
}

// this function rejects strings with leading zeros
pub fn unfixed_width<T: Number + FromStr>(s: &str) -> Option<T> {
    if !has_only_numbers(s) {
        return None;
    }

    if s.len() == 0 {
        return None;
    }

    if s.len() >= 2 {
        if s.starts_with('0') {
            return None;
        }
    }

    return s.parse().ok();
}

fn has_only_numbers(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}
