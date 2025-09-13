use rocket::*;
mod task;
use task::*;

use rocket::serde::json::Json;
use rocket::http::Status;

// #[get("/")]
// fn hello_user() -> & 'static str {
//     "Hello, All!"
// }

#[get("/tasks")]
fn fetch_tasks() -> Json<Vec<Task>> {
    let tasks = load_tasks();
    Json(tasks)
}

#[post("/create-tasks", format = "json", data = "<task>")]
fn add_task(task: Json<Task>) -> Status {
    let mut tasks = load_tasks();
    if let Some(existing_task) = tasks.iter().find(|t| t.task_name == task.task_name) {
        eprintln!("Task with name '{}' already exists.", existing_task.task_name);
        return Status::Conflict; // 409 Conflict
    }
    tasks.push(task.into_inner());
    save_tasks(&tasks);
    Status::Created
}

#[put("/tasks/<_index>", data = "<task>")]
fn update_task(_index: usize, task: Json<Task>) -> Status {
    let mut tasks = load_tasks();
    if _index < tasks.len() {
        tasks[_index] = task.into_inner();
        save_tasks(&tasks);
        Status::Ok
    } else {
        Status::NotFound
    }
}

#[delete("/tasks/<_index>")]
fn delete_task(_index: usize) -> Status {
    let mut tasks = load_tasks();
    if _index < tasks.len() {
        tasks.remove(_index);
        save_tasks(&tasks);
        Status::Ok
    } else {
        Status::NotFound
    }
}

#[launch]
fn rocket()-> _ {
    rocket::build().mount("/", routes![fetch_tasks, add_task, update_task])
}
