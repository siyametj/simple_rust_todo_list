// src/to_do_list.rs

pub struct ToDoList {
    pub tasks: Vec<String>,
}

impl ToDoList {
    pub fn new() -> Self {
        ToDoList { tasks: Vec::new() }
    }

    pub fn add_new_task(&mut self, task: String) {
        self.tasks.push(task);
        println!("Task added successfully!");
    }

    pub fn view_all_task(&self) {
        if self.tasks.is_empty() {
            println!("No task added yet!");
            return;
        }

        println!("\n----- Your Task -----");
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}. {}", index + 1, task);
        }
        println!("----------------------------");
    }

    pub fn delete_task(&mut self, user_index: usize) {
        // if self.tasks.is_empty() {
        //     println!("No task added yet!");
        //     return;
        // }

        if user_index > 0 && user_index <= self.tasks.len() {
            let actual_index = user_index - 1;
            let removed_task = self.tasks.remove(actual_index);
            println!("'{}' removed successfully!", removed_task);
        } else {
            println!("Tasks number not in list!")
        }
    }
}
