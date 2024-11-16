use std::io::{stdin, stdout, Write};
use std::fmt;


pub enum Command {
    Add,
    Edit,
    List,
    Remove,
    Update,
    Exit,
    Unknown
}
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
    Unknown
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            Self::NotStarted => "Not Started",
            Self::InProgress => "In Progress",
            Self::Completed => "Completed",
            Self::Unknown => "Unknown"
        };
        write!(f, "{}", status_str)
    }
}

pub fn input(text: &str, err_message: Option<&str>) -> String {
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

    return s;
}

pub fn get_command(args: Vec<&str>) -> Command {
    let command_str = args[1];
    let command = match command_str {
        "add" => Command::Add,
        "edit" => Command::Edit,
        "list" => Command::List,
        "remove" => Command::Remove,
        "update" => Command::Update,
        "Exit" => Command::Exit,
        &_ => Command::Unknown
    };

    return command;
}

pub fn get_specifier(args: Vec<&str>) -> String {
    return String::from(args[2]);
}

pub fn get_new_status(args: Vec<&str>) -> Status {
    let status_str = args[3];
    let status = match status_str {
        "not_started" => Status::NotStarted,
        "in_progress" => Status::InProgress,
        "completed" => Status::Completed,
        &_ => Status::Unknown
    };

    return status;
}