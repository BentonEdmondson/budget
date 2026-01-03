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
use clap::{Parser, Subcommand};
use limits::Limits;
use money::Money;
use std::fs::File;
use tag::Tag;
use transaction::Transactions;
use transaction_tree::TransactionTree;
use std::path::PathBuf;
use crate::date::Date;

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

fn main() {
    let _ = Command::parse();
    //dbg!("{}", command);
    
    // match command.as_str() {
    //     "init" => {
    //         commands::init::init(args);
    //         return;
    //     }
    //     "add" => {
    //         commands::add::add(args);
    //         return;
    //     }
    //     _ => (),
    // }
}
