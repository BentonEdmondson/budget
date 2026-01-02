use std::{env::Args, fs, io::{self, BufWriter}, path::Path, time::Instant};

use crate::transaction::Transactions;

pub fn init(mut args: Args) -> io::Result<()> {
    let start = Instant::now();

    if args.next().is_some() {
        panic!("Don't provide additional arguments to init")
    }

    let archive_path = Path::new("archive");
    let transactions_path = Path::new("transactions.json");

    let transactions = Transactions::empty();

    // TODO: convert these to errors
    match (archive_path.exists(), transactions_path.exists()) {
        (true, true) => panic!(
            "error: archive/ and transactions.json already exist; maybe you already ran budget init?"
        ),
        (true, false) => panic!("error: archive/ already exists; move or delete it"),
        (false, true) => panic!("error: transactions.json already exists; move or delete it"),
        (false, false) => (),
    }

    let transactions_file_unbuffered = fs::File::create_new(transactions_path)?;
    let transactions_file = BufWriter::new(transactions_file_unbuffered);
    transactions.to_json_writer(transactions_file)?;
    fs::create_dir(archive_path)?;

    let elapsed = start.elapsed();
    println!("Ran budget init in {} us", elapsed.as_micros());

    Ok(())
}
