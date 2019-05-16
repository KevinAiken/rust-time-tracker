use std::error::Error;

use chrono::prelude::*;

use rusqlite::{Connection};
use rusqlite::NO_PARAMS;

pub struct TimesheetEntry {
    pub activity: String,
    pub entry_time: String,
    pub input_type: String,
}

impl TimesheetEntry {
    pub fn new(args: &[String]) -> Result<TimesheetEntry, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let activity = args[2].clone();
        let input_type = args[1].clone();
        let entry_time: DateTime<Utc> = Utc::now();
        let entry_time = entry_time.to_string();

        Ok(TimesheetEntry { activity, entry_time, input_type })
    }
}

pub fn run(entry: TimesheetEntry) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("rust-time-tracker.db")?;

    conn.execute(
        "create table if not exists timesheet (\
            activity text,\
            entryTime text primary key,\
            inputType text\
            )",
        NO_PARAMS,
    )?;

    conn.execute(
        "INSERT INTO timesheet (activity, entryTime, inputType) VALUES (?1, ?2, ?3)",
        &[&entry.activity, &entry.entry_time, &entry.input_type],
    )?;

    println!("Entry added successfully.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute("create table if not exists timesheet (\
            activity text,\
            entryTime text primary key,\
            inputType text\
            )",
                     NO_PARAMS,
        )?;

        conn.execute(
            "INSERT INTO timesheet (activity, entryTime, inputType) VALUES (?1, ?2, ?3)",
            &["Example", "2019-05-16 16:33:07.541017400 UTC", "stop"],
        )?;
    }
}