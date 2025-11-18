use std::cmp::max;
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
    resource_needs: Option<BTreeMap<Resource, Vec<isize>>>,
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

    fn find_first_resource_calendar_compatibility (&self, resource_needs: BTreeMap<Resource, Vec<isize>>) -> Option<usize> {
        let mut first_compatible_intervals: BTreeMap<Resource, usize> = BTreeMap::new();
        let mut min_interval = 0;

        loop {
            for (resource, needs) in resource_needs.iter() {
                first_compatible_intervals.insert(resource.clone(), find_first_compatible_resource_window(&self.calendar[*resource], needs, min_interval)?);
            }

            let mut first_result = true;
            let mut all_results_same = true;

            for (_, &result) in first_compatible_intervals.iter() {
                if first_result {
                    first_result = false;
                    min_interval = result;
                } else if min_interval != result {
                    all_results_same = false;
                    min_interval = max(min_interval, result);
                }
            }

            if all_results_same {
                return Some(min_interval);
            }
        }

        None
    }
}

fn find_first_compatible_resource_window (resource_calendar: &Vec<isize>, resource_need: &Vec<isize>, starting_interval: usize) -> Option<usize> {
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

struct Configuration_Calendar {
    
}