use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::ErrorKind;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    pub id: u32,
    pub task: String,
    pub completed: bool,
}

pub fn load_from_csv() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    // Open the file. If it doesn't exist, just return an empty list 
    // instead of an error, because a fresh app won't have a file yet.
    let file = match OpenOptions::new().read(true).open("todos.csv") {
        Ok(file) => file,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e.into()),
    };

    let mut rdr = ReaderBuilder::new()
        .has_headers(false) // Set to true if your CSV has "id,task,completed" as the first line
        .from_reader(file);

    let list: Vec<Todo> = rdr.deserialize().collect::<Result<_, _>>()?;
    
    println!("Successfully loaded {} todos from disk.", list.len());
    Ok(list)
}

pub fn save_to_csv(todo: &Todo) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true) // Create the file if it doesn't exist
        .open("todos.csv")?;
    
    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
        
    wtr.serialize(todo)?;
    wtr.flush()?; // Ensure data is physically written to disk
    Ok(())
}

pub fn save_all_to_csv(todos: &[Todo]) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("todos.csv")?;

    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    for todo in todos {
        wtr.serialize(todo)?;
    }
    wtr.flush()?;
    Ok(())
}
