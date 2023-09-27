use sqlite::{Connection, State};

use crate::task::{Task};


pub struct DataBase {
  connection: Connection
}

impl DataBase {
  pub fn new() -> Self {
    let c = sqlite::open(":memory:").unwrap();
    let query = "CREATE TABLE todos (text TEXT, done INT)";
    c.execute(query).unwrap();
    DataBase { connection: c}
  }

  pub fn save_task(task: Task) {
    let query = "INSERT INTO todo where";
  }

  pub fn get_tasks(&mut self) -> Vec<Task> {
    let query = "SELECT * FROM todos";
    let mut tasks: Vec<Task> = vec![];
    let mut statment = self.connection.prepare(query).unwrap();
    while let Ok(State::Row) = statment.next() {
      let text = statment.read::<String,_>("text").unwrap();
      tasks.push(Task::new(text));
    }

    tasks
  }
}