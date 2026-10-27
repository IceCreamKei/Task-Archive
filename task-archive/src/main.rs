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
}

impl Task{
    pub fn new_task(id: usize, desc: String) -> Self{
        Self { 
            id, 
            desc, 
            task_type: (TaskType::Standard) }
    }
}

fn main() {
    let sample_task = Task::new_task(
        1,
        "Get Some Exercise".into()
    );
    println!("{:?}", sample_task);
}
