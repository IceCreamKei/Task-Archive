#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TaskType {
    Standard, // Tasks that only need to be checked through boolean
    Numbered {goal: u32, progress: u32} // Quantifiable Tasks (Drink "8" cups of water)
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub desc: String,
    pub task_type: TaskType,
    pub completed: bool,
}

impl Task{
    pub fn new_task(id: usize, desc: String, task_type: TaskType) -> Self{
        Self { 
            id, 
            desc, 
            task_type,
            completed: false, // Incomplete by default
        }
    }
}

// TaskManager is used to compile the tasks. Can be considered the core program.
pub struct TaskManager {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    // This function adds a task with its specific type/payload
    pub fn add(&mut self, description: String, task_type: TaskType) {
	    let task = Task::new_task(self.next_id, description, task_type);
	    self.next_id += 1;
	    self.tasks.push(task);
	}

    // This function progresses or finishes a task. (Standard is marked complete. Numbered increments by 1 until goal is reached)
    pub fn progress(&mut self, id: usize) -> Result<(), String> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| format!("Task with ID {} not found.", id))?;

        match &mut task.task_type {
            TaskType::Standard => {
                task.completed = true;
            }
            TaskType::Numbered { goal, progress } => {
                if *progress < *goal {
                    *progress += 1;
                }
                if *progress >= *goal {
                    task.completed = true;
                }
            }
        }

        Ok(())
    }

    // This function removes a task
    pub fn remove(&mut self, id: usize) -> Result<Task, String> {
        let index = self
            .tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| format!("Task with ID {} not found.", id))?;

        Ok(self.tasks.remove(index))
    }
	// This function fetches a task
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }
}

fn main() {
    let mut test = TaskManager::new();
    test.add(
        "Get Some Exercise".into(),
        TaskType::Standard,
    );
    test.add(
        "Drink Water".into(),
        TaskType::Numbered {
            goal: 8,
            progress: 0,
        },
    );
    println!("{:#?}", test.tasks());
}
