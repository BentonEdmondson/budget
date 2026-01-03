use crate::date::Date;
use crate::{Money, Tag, tag::TagSlice};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    io::{Read, Write},
};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Transaction {
    pub date: Date,
    pub amount: Money,
    pub tag: Tag,
    pub comment: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Transactions {
    pub transactions: Vec<Transaction>,
}

impl Transactions {
    pub fn from_json_reader<R>(r: R) -> Result<Self, serde_json::Error>
    where
        R: Read,
    {
        let mut transactions: Self = serde_json::from_reader(r)?;
        transactions.transactions.sort();

        return Ok(transactions);
    }

    pub fn to_json_writer<W>(&self, w: W) -> Result<(), serde_json::Error>
    where
        W: Write,
    {
        serde_json::to_writer_pretty(w, self)
    }

    pub fn empty() -> Transactions {
        Transactions {
            transactions: Vec::new(),
        }
    }

    pub fn add(&mut self, t: Transaction) {
        self.transactions.push(t);
        self.transactions.sort();
    }
}

impl<'a> Transaction {
    pub fn tag(&'a self) -> TagSlice<'a> {
        return self.tag.as_slice();
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({} on {})", self.comment, self.amount, self.date)
    }
}
