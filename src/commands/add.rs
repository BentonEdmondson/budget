use std::error::Error;
use std::str::FromStr;
use std::{env::Args, fs::File};

use crate::arg_parsers;
use crate::commands::CommandError;
use crate::date::Date;
use crate::money::Money;
use crate::tag::Tag;
use crate::transaction::{Transaction, Transactions};
use std::fs::OpenOptions;
use std::io::BufWriter;

pub fn add(
    tag: Tag,
    amount: Money,
    date: Date,
    comment: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let file = File::open_buffered("transactions.json")?;
    let mut transactions = Transactions::from_json_reader(file)?;

    let transaction = Transaction {
        tag,
        amount,
        comment: comment.unwrap_or("".to_string()),
        date,
    };

    transactions.add(transaction);

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("transactions.json")?;
    let file = BufWriter::new(file);
    transactions.to_json_writer(file)?;

    Ok(())
}
