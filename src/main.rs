use std::collections::{BTreeMap, HashMap};

type TaskId = usize;
type Configuration = usize;
type Resource = usize;

fn main() {
    println!("Hello, world!");
}

struct Schedule {
    tasks: HashMap<TaskId, Task>,
    ties: HashMap<TaskId, Vec<TaskId>>,
    resource_calendars: Option<HashMap<String, Resource_Calendar>>,
    config_calendar: Option<Configuration_Calendar>,
}

struct Task {
    task_id: TaskId,
    task_name: String,
    du: usize,
    early_start: usize,
    early_finish: usize,
    late_start: usize,
    late_finish: usize,
    manual_start: usize,
    resource_needs: Option<BTreeMap<Resource, usize>>,
    config_needs: Option<Vec<Configuration>>,
    config_latch: Option<Vec<Configuration>>,
    config_unlatch: Option<TaskId>,
}

struct Resource_Calendar {

}

struct Configuration_Calendar {
    
}