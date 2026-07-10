// src/main.rs

mod helper;
mod input_handler;
mod to_do_list;
use to_do_list::ToDoList;

fn main() {
    let mut my_task = ToDoList::new();

    loop {
        println!("\n1. Add Task  |  2. View Tasks  |  3. Delete Task  |  4. Exit");
        let choice = input_handler::input_number("Enter your choice");

        match choice {
            1 => {
                let task = input_handler::input_string("Enter task");
                my_task.add_new_task(task);
            }

            2 => {
                helper::clear_screen();
                my_task.view_all_task();
            }

            3 => {
                if !my_task.tasks.is_empty() {
                    my_task.view_all_task();

                    let task_number = input_handler::input_number("Enter task number") as usize;
                    my_task.delete_task(task_number);
                } else {
                    println!("No task added yet! Nothing to delete");
                }
            }

            4 => {
                println!("Goodbye! See you soon");
                break;
            }

            _ => {
                println!("Invalid choice! Please try again!");
            }
        }
    }
}
