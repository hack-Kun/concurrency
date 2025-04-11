use anyhow::{anyhow, Ok, Result};
use std::sync::mpsc::{self, channel};
use std::thread;
use std::time::Duration;

// 声明生产者数量
const PRODUCERS: usize = 3;

fn main() -> Result<()> {
    // 根据生产者数量创建线程和channel tx
    let (tx, rx) = channel();
    for i in 0..PRODUCERS {
        // 创建线程
        let tx = tx.clone();
        thread::spawn(move || product(i, tx));
    }
    // 生产结束释放tx
    drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("{:?}", msg);
        }
    });

    consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;
    Ok(())
}

fn product(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        // 生成数据 idx为线程编号， value为随即生成的值
        let value = rand::random::<u32>() as usize;
        tx.send(Msg::new(idx, value))?;
        // 指定线程的休眠时间
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        // 决定退出生产的条件
        if rand::random::<u8>() % 5 == 0 {
            println!("Thread {} exit", idx);
            break;
        }
    }
    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
struct Msg {
    id: usize,
    value: usize,
}

impl Msg {
    fn new(id: usize, value: usize) -> Self {
        Self { id, value }
    }
}
