use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub task_name: String,
    pub task_description: String,
    pub task_complete: String,
}

pub fn load_tasks() ->Vec<Task> {
   let mut tasks: Vec<Task> = vec![];
   let mut file: File = File::open("tasks.csv").unwrap_or_else(|_| File::create("tasks.csv").unwrap()); // of open fails then closure will run otherwise not // closure takes error but we dont need to care about error 
    let mut contents: String = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_)=> {
            let mut reader= Reader::from_reader(contents.as_bytes());
            for result in reader.deserialize() {
                let task: Task = result.unwrap();
                tasks.push(task);
                
            }

        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return tasks;
        }
    }

    tasks

}
pub fn save_tasks(tasks: &Vec<Task>)  {
let  file: File = File::create("tasks.csv").unwrap();
let mut wtr = Writer::from_writer(file);
for task in tasks {
    wtr.serialize(task).unwrap();
}
}