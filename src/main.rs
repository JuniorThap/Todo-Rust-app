extern crate rusqlite;
extern crate chrono;

use rusqlite::{params, Connection, Result};

use chrono::Local;

mod display;
mod command;
use crate::display::DataHolder;
use crate::command::{input, Command, get_command, get_specifier};


fn init_database() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("todo.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT DEFAULT 'No Description',
            created_at TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'Not Started'
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
    loop {
        let input = input("$ ", Some("Erorror"));
        let args: Vec<&str> = input.split_whitespace().collect();
        
        if args.len() < 2 {
            println!("Usage: todo <command> [args]");
            continue;
        }

        if args[0] != "todo" {
            println!("Usage: todo <command> [args]");
            continue;
        }
    
        // Get the command after todo e.g. add, list, remove
        let command = get_command(args.clone());
    
        // Handling the commands
        match command {
            Command::Add => {
                if args.len() < 3 {
                    println!("Usage: todo add <task_name>");
                    continue;
                }

                let task_name = get_specifier(args);
                let now = Local::now();
                let formatted = now.format("%d-%m-%Y %H:%M").to_string();
                
                conn.execute(
                    "INSERT INTO tasks (name, created_at) VALUES (?1, ?2)",
                    params![task_name, formatted]
                )?;
    
                println!("Added task: {}", task_name);
            },

            Command::Edit => {

            },
            
            
            Command::List => {
                let mut table = display::init_table();

                let mut stmt = match conn.prepare("SELECT id, name, description, created_at, status FROM tasks") {
                    Ok(stmt) => stmt,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };
                println!("{}", "query");
                let task_iter = stmt.query_map([], |row| {
                    Ok((
                        row.get::<_, u32>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                    ))
                })?
                .collect::<Result<Vec<_>, _>>()?;
    
                table.add_data(task_iter);
                table.printstd();
            },
            Command::Remove => {
                if args.len() < 3{
                    println!("Usage: todo remove <task_name/task_id>");
                    continue;
                }
                let specifier = get_specifier(args);
                let _ = conn.execute("DELETE FROM tasks WHERE id = (?1) OR name = (?1)",
                params![specifier]);
                
            },
            Command::Update => {
            },
            Command::Exit => {
                return Ok(());
            }
            Command::Unknown => todo!()
        }
    }
}