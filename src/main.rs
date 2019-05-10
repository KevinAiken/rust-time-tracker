extern crate rusqlite;
extern crate chrono;

use chrono::prelude::*;

use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

use std::collections::HashMap;
use std::env;

#[derive(Debug)]
struct TimesheetEntry {
    activity: String,
    entry_time: String,
    input_type: String,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let entry = parse_entry(&args);

    println!("Adding entry to {} tracking for {} at {}", entry.input_type, entry.activity, entry.entry_time);

    let conn = Connection::open("rust-time-tracker.db")?;

    conn.execute(
        "create table if not exists timesheet (\
            activity text,\
            entryTime text primary key,\
            inputType text\
            )",
        NO_PARAMS,
    )?;



    Ok(())
}

fn parse_entry(args: &[String]) -> TimesheetEntry {
    let activity = args[2].clone();
    let input_type = args[1].clone();
    let entry_time: DateTime<Utc> = Utc::now();
    let entry_time = entry_time.to_string();

    TimesheetEntry { activity, entry_time, input_type }
}