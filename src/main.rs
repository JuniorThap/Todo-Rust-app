extern crate rusqlite;
extern crate chrono;

use std::io::{stdin, stdout, Write};
use std::env::args;
use rusqlite::{params, Connection, Result};
use chrono::prelude::*;
use std::fmt;


enum Status {
    NotStart,
    InProgress,
    Completed
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_str = match self {
            Status::NotStart => "Not Started",
            Status::InProgress => "In Progress",
            Status::Completed => "Completed",
        };
        write!(f, "{}", status_str)
    }
}

fn input(text: &str, err_message: Option<&str>) {
    let err_message = err_message.unwrap_or("Did not enter a correct string");

    let mut s = String::new();
    print!("{}", text);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect(err_message);
    
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }

    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
}

fn init_database() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("todo.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            created_at TEXT NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

fn main() -> Result<()> {

    // Initializing database and handling the error.
    let conn = match init_database() {
        Ok(connection) => connection,
        Err(e) => {
            println!("Error initializing database: {:?}", e);
            return Err(e);
        }
    };

    // Collect the arguments from command line
    let args: Vec<String> = args().collect();
    println!("{:?}", args);

    if args.len() < 2 {
        println!("Usage: todo <command> [args]");
        return Ok(());
    }

    // Get the command after todo e.g. add, list, remove
    let command = &args[1];

    // Handling the commands
    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: todo add <task_name>");
                return Ok(());
            }
            let mut stmt = conn.prepare("SELECT MAX(id) FROM tasks")?;
            let max_id: Option<u64> = stmt.query_row([], |row| row.get(0))?;
            let id_value: u64 = max_id.unwrap_or(0);
            let task_name = &args[2];
            let now = chrono::Local::now().to_string();
            let status = Status::NotStart.to_string();

            conn.execute(
                "INSERT INTO tasks (name, created_at, status) VALUES (?1, ?2, ?3)",
                params![id_value, task_name, now, status]
            )?;

            println!("Added task: {}", task_name);
            Ok(())
        },
        

        "list" => {
            let mut stmt = conn.prepare("SELECT id, name, description, created_at, status")?;
            let task_iter = stmt.query_map([], |row| {
                Ok((
                    row.get::<_, u32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            })?;

            println!("ID\tNAME\tDESCRIPTION\tCREATED_AT\tSTATUS");
            for task in task_iter {
                let (id, name, description, created_at, status) = task?;
                println!("{}\t{}\t{}\t{}\t{}", id, name, description, created_at, status)
            }
            Ok(())
        },
        "remove" => {
            Ok(())
        },
        "mark" => {
            Ok(())
        },
        &_ => todo!()
    }
}