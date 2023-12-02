use std::hint::black_box;

use hello::{task_id_self, task_priority_set, task_spawn};

const ITER: usize = 3;
const LONG_TIME: usize = 1 << 25;

fn spawn_task(name: &str, priority: i32) {
    let name_owned = name.to_string();
    task_spawn(name.as_ref(), priority, move || {
        for _ in 0..ITER {
            println!("Hello from task {} with id {}", name_owned, task_id_self());
            for _ in 0..LONG_TIME {
                black_box(0);
            }
        }
    })
    .unwrap();
}

fn main() {
    let priorities = [(102, 101, 100), (100, 101, 102), (100, 100, 102)];

    for (a, b, c) in priorities {
        println!("==== {}, {}, {} ====", a, b, c);
        task_priority_set(task_id_self(), 0).unwrap();
        spawn_task("t1", a);
        spawn_task("t2", b);
        spawn_task("t3", c);
        task_priority_set(task_id_self(), 200).unwrap();
    }
}
