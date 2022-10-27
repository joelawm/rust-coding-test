use clap::arg;
use std::fs::File;
use crate::db::{transaction::Transaction};

mod db;
mod engine;

/// Allows you to grab the argument from the command line
pub fn args() -> String{
    let command = clap::command!().arg(arg!(<file>).allow_invalid_utf8(true)).get_matches();

    // Not the most safe method of returning, I generally try not to use unwraps instead hadneling the result
    return command.value_of_lossy("file").unwrap().to_string()
}

/// Grab the data and return a List of transcaritons to manipulate.
pub fn get_data(filename: String) -> Vec<Transaction> {
    // Open the file
    let file = File::open(filename).unwrap();

    // Create a CSv reader
    let mut rdr = csv::ReaderBuilder::new().delimiter(b',').trim(csv::Trim::All).from_reader(file);

    // Go through line by line and manipulate the file
    let mut transactions: Vec<Transaction> = Vec::new();
    for result in rdr.deserialize() {
        let record: Transaction = result.unwrap_or_else(|e|{
            panic!("Failed to grab the transaction data: {:?}", e);
        });

        // Add the record
        transactions.push(record);
    }

    transactions
}

/// This runs the transactions from the payment engine
pub fn run_transaction(data: Vec<Transaction>) {
    let mut engine = engine::PaymentEngine::new();

    // Run each transaction
    for i in data {
        engine.run_transaction(&i);
    }

    // Final output
    engine.print_output();
}