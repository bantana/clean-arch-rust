use std::time::Instant;

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub desc: String,
    tags: Vec<String>,
    due:  Option<Instant>,
    done: bool,
}


impl Task {
    pub fn new<T: Into<String>>(description: T) -> Self {
        Task {
            desc: description.into(),
            tags: Vec::new(),
            due: None,
            done: false,
        }
    }

    pub fn finish(&mut self) {
        self.done = true;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn due(mut self, when: Instant) -> Self {
        self.due = Some(when);
        self
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    tasks: Vec<Task>,
}

impl User {
    pub fn new<T: Into<String>>(name: T) -> Self {
        User {
            name: name.into(),
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }
}

