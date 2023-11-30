use hello::{task_id_self, task_priority_set, task_spawn};

static ITERATION: usize = 20;

fn print_stuff() {
    println!("Hello World from Task: 0x{:x}", task_id_self());
}

fn main() {
    let tid = task_id_self();
    println!("Main tid: 0x{:x}", tid);
    task_priority_set(tid, 0).unwrap();
    (0..ITERATION).for_each(|i| {
        let tid = task_spawn("print_stuff", 90 - i as i32, print_stuff).unwrap();
        println!("Spawn Tid: 0x{:x}, p: {}", tid, 90 - i);
    });

    task_priority_set(tid, 200).unwrap();
}
