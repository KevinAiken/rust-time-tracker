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
        if input_type != String::from("start") && input_type != String::from("stop") {
            return Err("Not a valid input type");
        }
        let entry_time: DateTime<Utc> = Utc::now();
        let entry_time = entry_time.to_string();

        Ok(TimesheetEntry { activity, entry_time, input_type })
    }
}

pub fn run(entry: TimesheetEntry) -> Result<(), Box<dyn Error>> {
    let mut conn = Connection::open("rust-time-tracker.db")?;

    create_table(&mut conn);

    insert_entry(entry, &mut conn);

    println!("Entry added successfully.");

    Ok(())
}



fn create_table(conn: &mut Connection) {
    conn.execute(
        "create table if not exists timesheet (\
            activity text,\
            entryTime text primary key,\
            inputType text\
            )",
        NO_PARAMS,
    ).expect("Failed to create table");
}

fn insert_entry(entry: TimesheetEntry, conn: &mut Connection) {
    conn.execute(
        "INSERT INTO timesheet (activity, entryTime, inputType) VALUES (?1, ?2, ?3)",
        &[&entry.activity, &entry.entry_time, &entry.input_type],
    ).expect("Failed to insert entry");
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::process;

    #[test]
    fn one_result() {
        let mut conn = Connection::open_in_memory().unwrap();

        let args = [String::from("Example"),
            String::from("start"), String::from("blah")];

        let entry = TimesheetEntry::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

        create_table(&mut conn);

        insert_entry(entry, &mut conn);

        ()
    }

    #[test]
    #[should_panic]
    fn fail_if_invalid_input_type() {
        let args = [String::from("Example"),
            String::from("wrongArg"), String::from("blah")];

        let entry = TimesheetEntry::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            panic!("Error creating timesheetentry");
        });
    }
}