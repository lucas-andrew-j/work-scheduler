use std::collections::HashMap;

type TaskId = usize;

fn main() {
    println!("Hello, world!");
}

struct Schedule {
    tasks: HashMap<TaskId, Task>,
    prioritized_tasks: Vec<TaskId>,
    ties: HashMap<TaskId, Vec<TaskId>>,
    resource_calendars: Option<HashMap<String, Resource_Calendar>>,
    config_calendar: Option<Configuration_Calendar>,
}

struct Task {
    task_id: TaskId,
    task_name: String,
    priority: usize,
    du: usize,
    scheduled: bool,
    early_start: usize,
    early_finish: usize,
    late_start: usize,
    late_finish: usize,
    manual_start: usize,
    resource_needs: Option<HashMap<String, Resource_Calendar>>,
    config_needs: Option<Configuration_Calendar>,
}

struct Resource_Calendar {

}

struct Configuration_Calendar {
    
}