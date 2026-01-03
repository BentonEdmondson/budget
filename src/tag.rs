use serde::{Deserialize, Deserializer, de::Error};
use serde::{Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

use crate::colors;

// TODO: use std::path::ancestors as inspiration for this object

#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
pub struct Tag {
    categories: Vec<String>,
}

#[derive(Debug)]
pub enum TagError {
    InvalidCharacters,
    EmptySegment,
}

impl FromStr for Tag {
    type Err = String;

    fn from_str(s: &str) -> Result<Tag, Self::Err> {
        if s == "." {
            return Ok(Tag {
                categories: Vec::new(),
            });
        }

        let categories: Vec<String> = s.split('.').map(str::to_owned).collect();

        let segment_is_empty = categories.iter().any(String::is_empty);
        if segment_is_empty {
            return Err("the tag contains an empty component".to_string());
        }

        let chars_are_valid = categories.iter().all(|segment| {
            segment
                .chars()
                .all(|char| char.is_alphanumeric() || char == '-')
        });
        if !chars_are_valid {
            return Err("the tag contains an invalid character".to_string());
        }

        return Ok(Tag { categories });
    }
}

// TODO: implement Deref for the TagSlice and merge the methods
impl<'a> Tag {
    pub fn as_slice(&'a self) -> TagSlice<'a> {
        TagSlice {
            slice: &self.categories[..],
        }
    }
}

impl fmt::Display for TagError {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Trying to serialize a TagError; it needs to look nice!");
    }
}

impl<'a> fmt::Display for TagSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.slice.is_empty() {
            return write!(f, "{}∀{}", colors::YELLOW, colors::RESET);
        }

        write!(
            f,
            "{}{}{}",
            colors::YELLOW,
            self.slice.join("."),
            colors::RESET
        )
    }
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Tag::from_str(&s).map_err(D::Error::custom)
    }
}

impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.categories.join(".").as_str())
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Ord, PartialOrd, Copy)]
pub struct TagSlice<'a> {
    slice: &'a [String],
}

impl<'a> TagSlice<'a> {
    fn parent(&self) -> Option<TagSlice<'a>> {
        if self.slice.is_empty() {
            return None;
        }

        return Some(TagSlice {
            slice: &self.slice[..self.slice.len() - 1],
        });
    }

    pub fn parents(&self) -> TagParents<'a> {
        TagParents { state: Some(*self) }
    }

    pub fn depth(&self) -> usize {
        self.slice.len()
    }
}

pub struct TagParents<'a> {
    state: Option<TagSlice<'a>>,
}

impl<'a> Iterator for TagParents<'a> {
    type Item = TagSlice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            None => None,
            Some(tag) => {
                self.state = tag.parent();
                return self.state;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let tag = Tag::from_str("first.second.third").unwrap();
        let tag = tag.as_slice();
        assert_eq!(tag.depth(), 3);
        let parent = tag.parent().unwrap();
        assert_eq!(parent.depth(), 2);
        let mut parents = parent.parents();
        assert_eq!(parents.next().unwrap().depth(), 1);
        assert_eq!(parents.next().unwrap().depth(), 0);
        assert_eq!(parents.next(), None);
    }
}
