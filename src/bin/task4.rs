use hello::{task_spawn, MessageQueue, WAIT_FOREVER};

fn main() {
    let channel = MessageQueue::new(5).unwrap();
    let clonned = channel.clone();

    task_spawn("send", 90, move || {
        for i in 0..10 {
            let value = Some(format!(
                "Hello World From other task though message queue: i = {}",
                i
            ));
            clonned.send(value, WAIT_FOREVER, 0).unwrap();
        }
        clonned.send(None, WAIT_FOREVER, 0).unwrap();
    })
    .unwrap();

    task_spawn("rec", 90, move || {
        while let Some(rec) = channel.receive(WAIT_FOREVER).unwrap() {
            println!("rec: {}", rec);
        }
    })
    .unwrap();
}
