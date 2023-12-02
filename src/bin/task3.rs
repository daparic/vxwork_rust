use hello::{task_spawn, Semaphore, SemaphoreOption, WAIT_FOREVER};

static mut GLOBAL: isize = 0;
static ITERATION: usize = 10;

fn main() {
    let sem: Semaphore = Semaphore::new(SemaphoreOption::Q_FIFO, true).unwrap();
    sem.take(-1).unwrap();

    let sem_c = sem.clone();
    task_spawn("task1", 90, move || {
        for _ in 0..ITERATION {
            sem_c.take(WAIT_FOREVER).unwrap();
            unsafe {
                GLOBAL += 1;
                println!("I am the adder, and global = {}", GLOBAL)
            }
            sem_c.release().unwrap();
        }
    })
    .unwrap();

    task_spawn("task2", 90, move || {
        sem.release().unwrap();
        for _ in 0..ITERATION {
            sem.take(WAIT_FOREVER).unwrap();
            unsafe {
                GLOBAL -= 1;
                println!("I am the subber, and global = {}", GLOBAL)
            }
            sem.release().unwrap();
        }
    })
    .unwrap();
}
