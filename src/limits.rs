use crate::{Money, Tag};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct Limits {
    pub limits: HashMap<Tag, Money>,
}

impl Limits {
    pub fn from_json_reader<R>(r: R) -> Result<Self, serde_json::Error>
    where
        R: Read,
    {
        let limits: Self = serde_json::from_reader(r)?;
        return Ok(limits);
    }
}
