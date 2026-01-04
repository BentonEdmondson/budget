#![feature(file_buffered)]

mod arg_parsers;
mod colors;
mod commands;
mod date;
mod limits;
mod money;
mod number_parsers;
mod tag;
mod transaction;
mod transaction_tree;
use crate::date::Date;
use clap::{Parser, Subcommand};
use money::Money;
use std::error::Error;
use std::path::PathBuf;
use tag::Tag;

#[derive(Parser, Debug)]
#[command(author, version)]
struct Command {
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    Init,
    Open,
    Close,
    Add {
        tag: Tag,
        amount: Money,
        #[arg(long, short)]
        date: Option<Date>,
        #[arg(long, short)]
        comment: Option<String>,
    },
    Remove {
        tag: Tag,
        amount: Money,
        #[arg(long, short)]
        date: Option<Date>,
        #[arg(long, short)]
        comment: Option<String>,
    },
    Status,
    Reconcile {
        file: PathBuf,
    },
    Audit {
        file: PathBuf,
    },
}

fn main() -> () {
    let command = Command::parse();

    let result: Result<(), Box<dyn Error>> = match command.subcommand {
        Subcommands::Init => commands::init::init(),
        Subcommands::Add {
            tag,
            amount,
            date,
            comment,
        } => commands::add::add(tag, amount, date.unwrap_or(Date::today()), comment),
        Subcommands::Remove {
            tag,
            amount,
            date,
            comment,
        } => commands::remove::remove(tag, amount, date, comment),
        _ => panic!("encountered unimplemented command"),
    };

    if let Err(err) = result {
        println!("error: {}", err);
    }
}
