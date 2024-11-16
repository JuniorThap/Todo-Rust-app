use prettytable::{Table, Row, Cell};


pub fn init_table() -> Table {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID"),
        Cell::new("NAME"),
        Cell::new("DESCRIPTION"),
        Cell::new("CREATED_AT"),
        Cell::new("STATUS"),
    ]));

    table
}

pub trait DataHolder {
    fn add_data(&mut self, task_iter: Vec<(u32, String, String, String, String)>);
}
impl DataHolder for Table {
    fn add_data(&mut self, task_iter: Vec<(u32, String, String, String, String)>) {
        for task in task_iter {
            let (id, name, description, created_at, status) = task;
            self.add_row(Row::new(vec![
                Cell::new(&id.to_string()),
                Cell::new(&name),
                Cell::new(&description),
                Cell::new(&created_at),
                Cell::new(&status),
            ]));
        }
    }
}