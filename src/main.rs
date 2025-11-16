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

    pub fn find_first_compatible_resource_window (&self, resource: usize, resource_need: Vec<isize>) -> Result<usize, String> {
        Err("Not implemented".to_owned())
    }

    // Checks the provided resource calendar slice against resource needs. Returns the first
    // interval where there are enough resources available until the end of the resource calendar
    // slice. If there are enough resources available, this will be the first element (0). If there
    // aren't enough resources available at any point, this will one more than the last element.
    fn check_window_resource_availability(resource_cal_slice: Vec<isize>, resource_need: Vec<isize>) -> usize {
        let result: Vec<isize> = zip(resource_cal_slice, resource_need).map(|(a, b)| a - b).collect();

        for i in (0..result.len()).rev() {
            if result[i] < 0 {
                return i;
            }
        }

        0
    }
}

struct Configuration_Calendar {
    
}