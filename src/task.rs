
pub struct Task {
  done: bool,
  text: String 
}

impl Task {
  pub fn new(text: String) -> Self {
    Task { done: false, text: text}
  }

  pub fn from(partial: PartialTask) -> Task {
    if partial.done.is_none() || partial.text.is_none() {
      panic!("missing values");
    }
    Task { done: partial.done.unwrap(), text: partial.text.unwrap() }
  } 

  pub fn mark_done(&mut self) {
    self.done = true;
  }
}

pub trait TaskList {
    fn index_of(&self, task: Task) -> Option<usize>;
    fn find(&mut self, query: String) -> Result<&mut Task, &'static str> ;
    fn del(&mut self, query: String) -> Result<(),&'static str>;
}

impl TaskList for Vec<Task> {
  fn index_of(&self, task: Task) -> Option<usize> {
      self.iter().position(|x| x.text == task.text && x.done == task.done)
  }

  fn find(&mut self, query: String) -> Result<&mut Task, &'static str> {
    match query.parse::<usize>() {
      Ok(numero) => {
        if numero < self.len() {
          Ok(&mut self[numero])
        } else {
          Err("indice fuera de rango")
        }
      },
      Err(_) => {
        if let Some(task) = self.iter_mut().find(|task| task.text == query) {
          Ok(task)
      } else {
          Err("Tarea no encontrada")
      }
      },
    }
  
  }
  fn del(&mut self, query: String) -> Result<(),&'static str> {
    match query.parse::<usize>() {
      Ok(numero) => {
        if numero < self.len() {
          self.remove(numero);
          Ok(())
        } else {
          Err("indice fuera de rango")
        }
      },
      Err(_) => {
        if let Some(index) = self.iter_mut().position(|task| task.text == query) {
          self.remove(index);
          Ok(())
      } else {
          Err("Tarea no encontrada")
      }
      },
    }
  }

}

pub fn list_tasks(tasks: &Vec<Task> ) {
    for task in tasks {
      let check = if task.done { "✓" } else { " " };
      println!("[{}] {}",check,task.text);
    }
}


pub struct PartialTask {
  pub text: Option<String>,
  pub done: Option<bool>
}

impl PartialTask {
  pub fn to_task(self) -> Result<Task, &'static str> {
    if self.text.is_some() && self.done.is_some() {
      let task = Task::from(self);
       Ok(task)
    } else {
      Err("Missing value")
    }
  }
}