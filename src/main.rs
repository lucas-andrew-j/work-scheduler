use std::collections::{BTreeMap, HashMap};
use std::iter::zip;

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
    config_compatibility: Vec<Vec<bool>>,
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
    resource_needs: Option<BTreeMap<Resource, isize>>,
    config_needs: Option<Vec<Configuration>>,
    config_latch: Option<Vec<Configuration>>,
    config_unlatch: Option<TaskId>,
}

struct Resource_Calendar {
    calendar: Vec<Vec<isize>>,
}

impl Resource_Calendar {
    fn new() -> Resource_Calendar {
        Resource_Calendar { calendar: Vec::new() }
    }

    pub fn add_resource_availability(&mut self, resource_availability: Vec<isize>) -> Resource {
        self.calendar.push(resource_availability);

        self.calendar.len() - 1
    }

    fn find_first_resource_calendar_compatibility (&self, resource_needs: BTreeMap<Resource, isize>) -> Option<usize> {
        

        None
    }

    fn find_first_compatible_resource_window (resource_calendar: &Vec<isize>, resource_need: Vec<isize>) -> Option<usize> {
        'outer: for i in 0..resource_calendar.len() - resource_need.len() {
             for j in 0..resource_need.len() {
                 if resource_calendar[i + j] < resource_need[j] {
                     continue 'outer;
                 }
             }
            return Some(i);
            }
        None
    }
}

struct Configuration_Calendar {
    
}