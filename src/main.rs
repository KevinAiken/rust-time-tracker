extern crate rusqlite;
extern crate chrono;

use rust_time_tracker::TimesheetEntry;

use std::{env, process};
use std::error::Error;

fn main(){
    let args: Vec<String> = env::args().collect();

    let entry = TimesheetEntry::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Adding entry to {} tracking for {} at {}", entry.input_type, entry.activity, entry.entry_time);

    if let Err(e) = rust_time_tracker::run(entry) {
        println!("Application error: {}", e);

        process::exit(1);
    }


}


