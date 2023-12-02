use hello::{
    mq::{MessageQueue, MessageQueuePriority},
    task_spawn, WAIT_FOREVER,
};

fn main() {
    let channel = MessageQueue::new(5).unwrap();
    let ack_channel = MessageQueue::new(5).unwrap();

    let clonned = channel.clone();
    let clonned_ack = ack_channel.clone();

    task_spawn("send", 90, move || {
        for i in 0..10 {
            let value = Some(format!(
                "Hello World From other task though message queue: i = {}",
                i
            ));
            clonned
                .send(value, WAIT_FOREVER, MessageQueuePriority::Normal)
                .unwrap();

            let ack = clonned_ack.receive(WAIT_FOREVER).unwrap();
            if ack {
                println!("send: ack from rec")
            }
        }
        clonned
            .send(None, WAIT_FOREVER, MessageQueuePriority::Normal)
            .unwrap();
    })
    .unwrap();

    task_spawn("rec", 90, move || {
        while let Some(rec) = channel.receive(WAIT_FOREVER).unwrap() {
            println!("rec: {}", rec);

            ack_channel
                .send(true, WAIT_FOREVER, MessageQueuePriority::Normal)
                .unwrap();
        }
    })
    .unwrap();
}
