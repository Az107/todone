use std::io::{self, BufRead, Write};

mod task;
use task::Task;

mod command;
use command::handle_command;

mod dataBase;


fn main() {
    let stdin = io::stdin();
    let mut tasks: Vec<Task> = Vec::new();
    let mut iterator = stdin.lock().lines();
    println!("Welcome");
    loop {
      print!(">");
      let _ = io::stdout().flush();
      let line = iterator.next().unwrap().unwrap();
      let cmd: Vec<_> = line.split_whitespace().collect();
      if handle_command(&cmd, &mut tasks) {
        break;
      }
    
    }
    println!("Bye!");
  }