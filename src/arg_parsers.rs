use std::{collections::HashMap, env::Args};

#[derive(Debug)]
pub enum ArgError {
    FoundFlagWithNoDash,
    FoundValueWithDash,
    ValueIsMissing,
}

pub fn options(mut args: Args) -> Result<HashMap<String, String>, ArgError> {
    let mut result = HashMap::new();

    loop {
        let first = match args.next() {
            None => return Ok(result),
            Some(value) => value,
        };

        match args.next() {
            None => return Err(ArgError::ValueIsMissing),
            Some(second) => {
                match (first.starts_with("-"), second.starts_with("-")) {
                    (true, false) => result.insert(first, second),
                    (false, _) => return Err(ArgError::FoundFlagWithNoDash),
                    (_, true) => return Err(ArgError::FoundValueWithDash),
                };
            }
        }

        return Ok(result);
    }
}
