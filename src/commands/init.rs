use crate::date::Date;
use std::error::Error;
use std::{
    env::Args,
    fs,
    io::{self, BufWriter},
    path::Path,
    time::Instant,
};

use crate::transaction::Transactions;

pub fn init() -> Result<(), Box<dyn Error>> {
    let archive_path = Path::new("archive");
    let transactions_path = Path::new("transactions.json");

    let transactions = Transactions::empty();

    match (archive_path.exists(), transactions_path.exists()) {
        (true, true) => {
            return Err(
                "archive/ and transactions.json already exist; maybe you already ran budget init?"
                    .into(),
            );
        }
        (true, false) => return Err("archive/ already exists; move or delete it".into()),
        (false, true) => return Err("transactions.json already exists; move or delete it".into()),
        (false, false) => (),
    };

    let transactions_file_unbuffered = fs::File::create_new(transactions_path)?;
    let transactions_file = BufWriter::new(transactions_file_unbuffered);
    transactions.to_json_writer(transactions_file)?;
    fs::create_dir(archive_path)?;

    Ok(())
}
