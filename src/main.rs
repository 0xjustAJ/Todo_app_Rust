use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize, //the size will be defined by our computer architecture
    description: String,
    completed: bool,
}

#[derive(StructOpt, Debug)]
#[structopt(name="todo")]
enum Command {
    //add a new task
    Add { description: String},
    //list all task
    List,
    //Remove task by id
    Remove {id: usize},
    // mark a task complete by ID
    Complete {id: usize},
}

fn main() -> io::Result<()> {
   let cmd = Command::from_args();
   let mut tasks = load_task()?;

   match cmd {
    Command::Add { description } => {
        let id = tasks.len() + 1;
        tasks.push(Task {id, description, completed: false});
        save_tasks(&tasks)?;
        println!("Task added.");
    }

    Command::List => {
        for task in &tasks {
            println!("{}: {} [{}]", task.id, task.description, if task.completed {"x"} else {" "});
        }
    }

    Command::Remove{id} => {
        tasks.retain(|task| task.id != id);
        save_tasks(&tasks)?;
        println!("Task removed.");
    }
    Command::Complete {id} => {
        if let Some(task) = tasks.iter_mut().find(|task| task.id ==id) {
            task.completed =true;
            save_tasks(&tasks)?;
            println!("Task completed");
        }else {
            println!("Task not found");
        }
    }

   }
   Ok(())
}

//the we create the load and save function
fn load_task() -> io::Result<Vec<Task>> {
    let file = OpenOptions::new().read(true).open("tasks.json");
    if let Ok(mut file) = file {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let tasks: Vec<Task> = serde_json::from_str(&contents).unwrap_or_else(|_| vec![]);
        Ok(tasks)
    }else {
        Ok(vec![])
    }
}

fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let json= serde_json::to_string(tasks)?;
    let mut file = OpenOptions::new()
                               .write(true)
                               .create(true)
                               .truncate(true)
                               .open("tasks.json")?;
                            file.write_all(json.as_bytes())
}