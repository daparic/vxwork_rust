use hello::{sys_clock_rate, task_delay, task_id_self, task_spawn};

const PRIORITY: i32 = 101;

const ITER_OUTTER: usize = 100;
const ITER_INNER: usize = 10;

fn main() {
    let clock_rate = sys_clock_rate();
    println!("Sys Clock Rate: {}", clock_rate);
    // kernel_time_slice(clock_rate / 60).unwrap();

    task_delay(clock_rate * 2).unwrap();

    for i in 0..3 {
        let name = format!("task{}", i);
        task_spawn(name.as_str(), PRIORITY, || {
            for i in 0..ITER_OUTTER {
                for j in 0..ITER_INNER {
                    println!("Task: {}, i = {}, j = {}", task_id_self(), i, j);
                }
            }
        })
        .unwrap();
    }
}
