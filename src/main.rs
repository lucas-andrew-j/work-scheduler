use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Schedule {
    tasks: Vec<Task>,
    ties: Vec<Vec<usize>>,
    resource_calendars: Option<HashMap<String, Resource_Calendar>>,
    config_calendar: Option<Configuration_Calendar>,
}

struct Task {
    task_id: String,
    task_num: usize,
    priority: usize,
    du: usize,
    scheduled: bool,
    early_start: usize,
    early_finish: usize,
    late_start: usize,
    late_finish: usize,
    manual_start: usize,
    resource_calendars: Option<HashMap<String, Resource_Calendar>>,
    config_calendar: Option<Configuration_Calendar>,
}

struct Resource_Calendar {

}

struct Configuration_Calendar {
    
}