// Last updated: 18.09.2025, 22:45:50
use std::collections::{BTreeMap, HashMap};

struct TaskManager {
    task_id2priority: HashMap<i32, i32>,
    data: BTreeMap<(i32, i32), i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut task_id2priority = HashMap::with_capacity(tasks.len() + 200_000);
        let mut data = BTreeMap::new();
        for task in tasks {
            let [user_id, task_id, priority] = task[..] else {
                unimplemented!()
            };
            task_id2priority.insert(task_id, priority);
            data.insert((priority, task_id), user_id);
        }
        Self {
            task_id2priority,
            data,
        }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.task_id2priority.insert(task_id, priority);
        self.data.insert((priority, task_id), user_id);
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        let old_priority = self.task_id2priority.get_mut(&task_id).unwrap();
        let user_id = self.data.remove(&(*old_priority, task_id)).unwrap();
        self.data.insert((new_priority, task_id), user_id);
        *old_priority = new_priority;
    }

    fn rmv(&mut self, task_id: i32) {
        let priority = self.task_id2priority.remove(&task_id).unwrap();
        self.data.remove(&(priority, task_id));
    }

    fn exec_top(&mut self) -> i32 {
        if let Some((_, user_id)) = self.data.pop_last() {
            user_id
        } else {
            -1
        }
    }
}

/**
 * Your TaskManager object will be instantiated and called as such:
 * let obj = TaskManager::new(tasks);
 * obj.add(userId, taskId, priority);
 * obj.edit(taskId, newPriority);
 * obj.rmv(taskId);
 * let ret_4: i32 = obj.exec_top();
 */