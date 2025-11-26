use std::collections::{BTreeMap, BTreeSet, HashMap };

type TaskId = usize;
type Resource = usize;
type Shift = u8;
type Interval = usize;

fn main() {
    let mut tasks: HashMap<TaskId, Task> = HashMap::new();
    tasks.insert(1, Task {
        task_id: 1,
        task_name: "Task 1".to_owned(),
        du: 1,
        shifts: 1,
        early_start: None,
        early_finish: None,
        late_start: None,
        late_finish: None,
        manual_start: None,
        resource_needs: None,
    });
    tasks.insert(2, Task {
        task_id: 2,
        task_name: "Task 2".to_owned(),
        du: 1,
        shifts: 1,
        early_start: None,
        early_finish: None,
        late_start: None,
        late_finish: None,
        manual_start: None,
        resource_needs: None,
    });

    let mut ties: HashMap<TaskId, Vec<TaskId>> = HashMap::new();
    ties.insert(1, vec![2]);
    ties.insert(2, vec![1]);

    let schedule = Schedule::new(
        tasks,
        ties,
        1);

    println!("{}", schedule.contains_cycle());
}

#[derive(Debug)]
struct Schedule {
    tasks: HashMap<TaskId, Task>,
    ties: HashMap<TaskId, Vec<TaskId>>,
    resource_demand: BTreeMap<String, usize>,
    shifts: Shift,
}

#[derive(Debug)]
struct Task {
    task_id: TaskId,
    task_name: String,
    du: usize,
    shifts: Shift,
    early_start: Option<(Shift, Interval)>,
    early_finish: Option<(Shift, Interval)>,
    late_start: Option<(Shift, Interval)>,
    late_finish: Option<(Shift, Interval)>,
    manual_start: Option<(Shift, Interval)>,
    resource_needs: Option<Vec<(Resource, Vec<isize>)>>,
}

impl Schedule {
    fn new(tasks: HashMap<TaskId, Task>, ties: HashMap<TaskId, Vec<TaskId>>, shifts: u8) -> Schedule {
        Schedule {
            tasks: tasks,
            ties: ties,
            resource_demand: BTreeMap::new(),
            shifts: shifts,
        }
    }

    pub fn contains_cycle(&self) -> bool {
        let mut visited: BTreeSet<TaskId> = BTreeSet::new();

        // for task in self.tasks.iter() {
        //     if !self.ties.contains_key(&task.1.task_id) {
        //         tasks_without_preds.push(task.1.task_id);
        //     }
        // }

        let mut path: BTreeSet<TaskId> = BTreeSet::new();

        for task in self.tasks.iter() {
            if !visited.contains(&task.1.task_id) {
                if depth_first_cycle_check(task.1.task_id, &self.ties, &mut visited, &mut path) {
                    return true;
                }
            }
        }

        false
    }
}

fn depth_first_cycle_check(task_id: TaskId, ties: &HashMap<TaskId, Vec<TaskId>>, visited: &mut BTreeSet<TaskId>, path: &mut BTreeSet<TaskId>) -> bool {
    if path.contains(&task_id) {
        return true;
    } else {
        path.insert(task_id);
    }

    if let Some(tied_tasks) = ties.get(&task_id) {
        for task in tied_tasks {
            if depth_first_cycle_check(*task, ties, visited, path) {
                return true;
            }
        }
    } else {
        return false;
    }

    false
}

// impl Task {
//     fn new()
// }






