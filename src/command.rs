use crate::task::{Task, list_tasks, TaskList};




pub fn handle_command(cmd: &[&str], tasks: &mut Vec<Task>) -> bool {
  let mut result = false;
  match cmd[0] {
    "exit" => result = true,
    "list" => list_tasks(&tasks),
    "echo" => println!("{}", cmd[1]),
    "add"  => if cmd.len() > 1 {tasks.push(Task::new(cmd[1..].join(" ")))},
    "check" => if cmd.len() > 1 { 
      let result = tasks.find(cmd[1..].join(" "));
      if result.is_ok() {result.unwrap().mark_done()}
      else {println!("💥 {}",result.err().unwrap())}
    },
    "del" => if cmd.len() > 1 { 
      let result = tasks.del(cmd[1..].join(" "));
      if result.is_ok() {println!("Eliminado")}
      else {println!("💥 {}",result.err().unwrap())}
    },
    "help" => {
      println!("exit -> Sale del programa");
      println!("list -> muestra los todos (tareas)");
      println!("echo <texto> -> escribe el texto)");
      println!("check <tarea> -> marca una tarea como realizada");
      println!("del <tarea> -> elimina una tarea");

    }
    _ => println!("undefined command: {}\n Escribe help para ayuda", cmd[0])
  }
  result
}
