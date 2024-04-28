use std::{sync::mpsc, thread};

use anyhow::{anyhow, Result};

const NUM_PRODUCES: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

impl Msg {
    pub fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_PRODUCES {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx); // drop tx to close the channel
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Consumer: {:?}", msg);
        }
        42
    });
    let secret = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;
    println!("Secret: {}", secret);
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(std::time::Duration::from_millis(sleep_time));
        // random exit the producer
        if rand::random::<u8>() % 10 == 0 {
            println!("Producer {} exit", idx);
            break;
        }
    }
    Ok(())
}
