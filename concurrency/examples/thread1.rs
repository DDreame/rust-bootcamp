use anyhow::{anyhow, Result};
use std::{
    sync::mpsc,
    thread::{self, sleep},
    time::Duration,
};

static NUM_PROCESSORS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: u64,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for idx in 0..NUM_PROCESSORS {
        let tx = tx.clone();
        thread::spawn(move || producer(idx, tx));
    }

    drop(tx);
    let recv = thread::spawn(move || {
        for msg in rx {
            println!("{:?}", msg);
        }
    });
    recv.join()
        .map_err(|e| anyhow!("Thread join error {:?}", e))?;

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) {
    for value in 0..100 {
        let msg = Msg::new(idx, value);
        tx.send(msg).unwrap();
        sleep(Duration::from_millis(100));
    }
}

impl Msg {
    fn new(idx: usize, value: u64) -> Self {
        Self { idx, value }
    }
}
