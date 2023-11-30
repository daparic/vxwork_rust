use std::{thread, time::Duration};

use env_logger::WriteStyle;
use hello::{
    signal::{kill, signal_action, Signal},
    task_spawn, Semaphore, SemaphoreOption, WAIT_FOREVER,
};
use lazy_static::lazy_static;
use log::{debug, info, warn, LevelFilter};

lazy_static! {
    static ref SEMAPHORE: Semaphore = Semaphore::new(SemaphoreOption::Q_FIFO, false).unwrap();
}

extern "C" fn handle_sigint(sig: i32) {
    info!("Got sig: {}", sig);
}

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .write_style(WriteStyle::Always)
        .init();

    let tid = task_spawn("signal", 100, || {
        signal_action(Signal::SIGINT, 0, handle_sigint).unwrap();
        info!("Installed SIGINT handler");
        SEMAPHORE.release().unwrap();

        debug!("Waiting to simulate work!");
        thread::sleep(Duration::from_secs(10));
        warn!("Done all the work, should have been killed by now!")
    })
    .unwrap();

    SEMAPHORE.take(WAIT_FOREVER).unwrap();

    info!("Killing task");
    kill(tid, Signal::SIGINT).unwrap();
    info!("Killed task")
}
