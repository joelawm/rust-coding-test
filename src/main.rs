/*-------------
/main.rs

The main start to the program itself

Changelog:
--- Version 1.0 - Joe Meyer
	-- Initial code release.
-------------*/
use payment_engine::{args, get_data, run_transaction};

fn main() {
    // Set the logger
    log4rs::init_file("logging_config.yaml", Default::default()).expect("Logging file is missing.");

    // Main program start
    let data = get_data(args());

    run_transaction(data);
}
