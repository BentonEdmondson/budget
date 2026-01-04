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

pub fn remove(
    tag: Tag,
    amount: Money,
    date: Option<Date>,
    comment: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let file = File::open_buffered("transactions.json")?;
    let mut transactions = Transactions::from_json_reader(file)?;
    let original_count = transactions.transactions.len();

    transactions.transactions.retain(|t| {
        t.tag != tag
            || t.amount != amount
            || if let Some(date_) = &date {
                *date_ != t.date
            } else {
                false
            }
            || if let Some(comment_) = &comment {
                *comment_ != t.comment
            } else {
                false
            }
    });

    assert!(original_count >= transactions.transactions.len());
    let number_removed = original_count - transactions.transactions.len();

    match (number_removed, date, comment) {
        (0, _, _) => {
            return Err("no such transaction found".into())
        },
        // TODO: this should print the transaction that was found
        (1, _, _) => (),
        // TODO: these should print the transactions that were found
        (2.., None, None) => {
            return Err("multiple matching transactions found; try including the transaction's date or comment".into())
        },
        (2.., Some(_), None) => {
            return Err("multiple matching transactions found; try including the transaction's comment".into())
        },
        (2.., None, Some(_)) => {
            return Err("multiple matching transactions found; try including the transaction's date".into())
        },
        (2.., Some(_), Some(_)) => {
            return Err("multiple matching transactions found; manually remove the transaction from transactions.json".into())
        },
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("transactions.json")?;
    let file = BufWriter::new(file);
    transactions.to_json_writer(file)?;

    Ok(())
}
