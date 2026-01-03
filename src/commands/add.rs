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

pub fn add(mut args: Args, today: Date) -> Result<(), Box<dyn Error>> {
    let file = File::open_buffered("transactions.json")?;
    let mut transactions = Transactions::from_json_reader(file)?;

    let tag: Tag = args
        .next()
        .expect("tag was expected")
        .parse()
        .expect("invalid tag format");
    let amount: Money = args
        .next()
        .expect("money was expected")
        .parse()
        .expect("invalid money format");
    let options = arg_parsers::options(args).unwrap();
    let comment: Option<String> = options.get("-m").cloned();
    let date: Option<Date> = options.get("-d").map(|s| Date::from_str(s).unwrap());

    let transaction = Transaction {
        tag,
        amount,
        comment: comment.unwrap_or("".to_string()),
        date: date.unwrap_or(today),
    };

    transactions.add(transaction);

    let file = OpenOptions::new().write(true).open("transactions.json")?;
    let file = BufWriter::new(file);
    transactions.to_json_writer(file)?;

    Ok(())
}
