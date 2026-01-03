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
        money: String,
        #[arg(long, short)]
        comment: Option<String>,
        #[arg(long, short)]
        date: Option<Date>,
    },
    Remove {
        tag: Tag,
        money: String,
        #[arg(long, short)]
        comment: Option<String>,
        #[arg(long, short)]
        date: Option<Date>,
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
        _ => panic!("encountered unimplemented command"),
    };

    if let Err(err) = result {
        println!("error: {}", err);
    }
}
