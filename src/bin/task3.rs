use hello::{task_spawn, Semaphore, SemaphoreOption, WAIT_FOREVER};
use lazy_static::lazy_static;

lazy_static! {
    static ref SEMAPHORE: Semaphore = Semaphore::new(SemaphoreOption::Q_FIFO, true).unwrap();
}

static mut GLOBAL: isize = 0;
static ITERATION: usize = 10;

fn main() {
    SEMAPHORE.take(-1).unwrap();

    task_spawn("t1", 90, || {
        for _ in 0..ITERATION {
            SEMAPHORE.take(WAIT_FOREVER).unwrap();
            unsafe {
                GLOBAL += 1;
                println!("I am the adder, and global = {}", GLOBAL)
            }
            SEMAPHORE.release().unwrap();
        }
    })
    .unwrap();

    task_spawn("t2", 90, || {
        SEMAPHORE.release().unwrap();
        for _ in 0..ITERATION {
            SEMAPHORE.take(WAIT_FOREVER).unwrap();
            unsafe {
                GLOBAL -= 1;
                println!("I am the subber, and global = {}", GLOBAL)
            }
            SEMAPHORE.release().unwrap();
        }
    })
    .unwrap();
}
