#![allow(dead_code, unused)]

#[macro_use]
extern crate prettytable;

enum TaskPriority {
    Low,
    Medium,
    High
}

impl TaskPriority {
    fn print(&self) -> &str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}



#[derive(Debug)]
enum TaskStatus {
    Todo,
    Doing,
    Done,
}

impl TaskStatus {
    fn print(&self) -> &str {
        match self {
            Self::Doing => "doing",
            Self::Todo => "todo",
            Self::Done => "done",
        }
    }
}

#[derive(Debug)]
struct Task {
    title: String,
    description: String,
    status: TaskStatus,
    id: i32,
}

impl Task {
    fn change_task_status(&mut self, status: TaskStatus) -> &mut Self {
        self.status = status;
        self
    }

    fn change_task_title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_owned();
        self
    }

    fn change_task_description(&mut self, description: &str) -> &mut Self {
        self.description = description.to_owned();
        self
    }

    fn mark_task_as_done(&mut self) -> &mut Self {
        self.status = TaskStatus::Done;
        self
    }

    fn mark_task_as_todo(&mut self) -> &mut Self {
        self.status = TaskStatus::Todo;
        self
    }

    fn mark_task_as_doing(&mut self) -> &mut Self {
        self.status = TaskStatus::Doing;
        self
    }

    fn print(&mut self) -> &mut Self {
        let mut table = table!();
        table.add_row(row!["id", "status", "title", "description"]);
        table.add_row(row![
            self.id,
            self.status.print(),
            self.title,
            self.description
        ]);
        table.printstd();
        self
    }
}

#[derive(Debug)]
struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn add_task(&mut self, title: &str, description: &str, status: TaskStatus) {
        let last_task_id = self.tasks.last().map(|task| task.id);

        let id = match last_task_id {
            Some(id) => id,
            None => 0,
        };

        self.tasks.push(Task {
            title: title.to_owned(),
            description: description.to_owned(),
            status,
            id: id + 1,
        })
    }
    fn print_tasks(&self) {
        let mut table = table!();
        table.add_row(row!["id", "status", "title", "description"]);
        for task in self.tasks.iter() {
            table.add_row(row![
                task.id,
                task.status.print(),
                task.title,
                task.description
            ]);
        }
        table.printstd();
    }

    fn find_task(&mut self, id: i32) -> Option<&mut Task> {
        let task = self.tasks.iter_mut().find(|task| task.id == id);
        task
    }

    fn remove_task(&mut self, id: i32) -> &mut Self {
        self.tasks.retain(|t| t.id != id);
        self
    }

    fn new() -> Self {
        Self { tasks: Vec::new() }
    }
}

fn main() {
    let mut task_manager = TaskManager::new();

    task_manager.add_task(
        "finish task manager",
        "finish task manger for learn ownership model better",
        TaskStatus::Doing,
    );
    task_manager.add_task(
        "finish task manager",
        "finish task manger for learn ownership model better",
        TaskStatus::Doing,
    );
    task_manager.add_task(
        "finish task manager",
        "finish task manger for learn ownership model bette rfinish task manger for learn ownership model better",
        TaskStatus::Doing,
    );
    task_manager.find_task(1).unwrap().mark_task_as_done().print();
    task_manager.remove_task(1).print_tasks()
}
