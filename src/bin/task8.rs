use std::{thread, time::Duration};

use env_logger::WriteStyle;
use hello::{
    signal::{kill, signal_action, Signal},
    task_spawn, Semaphore, SemaphoreOption, WAIT_FOREVER,
};
use log::{debug, error, info, LevelFilter};

extern "C" fn handle_sigint(sig: i32) {
    info!("Got sig: {}", sig);
}

fn main() {
    let sem: Semaphore = Semaphore::new(SemaphoreOption::Q_FIFO, false).unwrap();

    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .write_style(WriteStyle::Always)
        .init();

    let sem_c = sem.clone();
    let tid = task_spawn("signal", 100, move || {
        signal_action(Signal::SIGINT, 0, handle_sigint).unwrap();

        info!("Installed SIGINT handler");
        sem_c.release().unwrap();

        debug!("Waiting to simulate work!");
        thread::sleep(Duration::from_secs(10));
        error!("Done all the work, should have been killed by now!")
    })
    .unwrap();

    sem.take(WAIT_FOREVER).unwrap();

    info!("Killing task");
    kill(tid, Signal::SIGINT).unwrap();
    info!("Killed task")
}
