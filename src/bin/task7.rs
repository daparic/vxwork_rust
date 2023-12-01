use std::hint::black_box;

use hello::{
    task_delay, task_id_self, task_priority_set, task_spawn, Semaphore, SemaphoreOption,
    WAIT_FOREVER,
};

const HIGH_PRIORITY: i32 = 100;
const MID_PRIORITY: i32 = 103;
const LOW_PRIORITY: i32 = 104;
const ITER: usize = 3;
const LONG_TIME: usize = 1 << 28;

fn main() {
    let option = SemaphoreOption::Q_PRIORITY | SemaphoreOption::INVERSION_SAFE;
    // let option = SemaphoreOption::Q_PRIORITY;
    println!("[MAIN] options: {}", option.bits());

    let sem = Semaphore::new_mutex(option).unwrap();

    task_priority_set(task_id_self(), 101).unwrap();

    let upper_name = "LOW ";
    println!("[MAIN] spawning task {}", upper_name);
    task_spawn(upper_name, LOW_PRIORITY, move || {
        for _ in 0..ITER {
            println!("[{}] Try to take mutex", upper_name);
            sem.take(WAIT_FOREVER).unwrap();
            println!("[{}] Takes mutex", upper_name);
            for _ in 0..LONG_TIME {
                black_box(0);
            }
            println!("[{}] Release mutex", upper_name);
            sem.release().unwrap();
        }
        println!("[{}] Done!", upper_name)
    })
    .unwrap();

    println!("[MAIN] Spawning task MID");
    task_spawn("mid", MID_PRIORITY, move || {
        task_delay(10).unwrap();
        for i in 0..LONG_TIME * 10 {
            if i % LONG_TIME == 0 {
                println!("[MID ] Running~")
            }
        }
        println!("[MID ] Completed!");
    })
    .unwrap();

    let upper_name = "HIGH";
    println!("[MAIN] spawning task {}", upper_name);
    task_spawn(upper_name, HIGH_PRIORITY, move || {
        task_delay(13).unwrap();
        for _ in 0..ITER {
            println!("[{}] Try to take mutex", upper_name);
            sem.take(WAIT_FOREVER).unwrap();
            println!("[{}] Takes mutex", upper_name);
            for _ in 0..LONG_TIME {
                black_box(0);
            }
            println!("[{}] Release mutex", upper_name);
            sem.release().unwrap();
        }
        println!("[{}] Done!", upper_name)
    })
    .unwrap();
    task_priority_set(task_id_self(), 200).unwrap();
}
