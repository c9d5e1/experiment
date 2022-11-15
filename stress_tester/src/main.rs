use std::env::args;
use std::future::Future;
use std::io;
use std::net::Shutdown;
use std::str::FromStr;
use std::time::{Duration, Instant};
use async_std::channel::{bounded, Receiver, Sender, unbounded};
use async_std::io::{ReadExt, WriteExt};
use async_std::net::TcpStream;
use async_std::task::{JoinHandle, sleep};

const ROOT_REQUEST: &str = "GET / HTTP/1.1\r\n\r\n";
const SLEEP_REQUEST: &str = "GET /sleep HTTP/1.1\r\n\r\n";
const AMOUNT: usize = 1000;

#[async_std::main]
async fn main() -> io::Result<()> {
    println!("Running!");
    let mut args = args();
    args.next();
    let task = args.next();
    match task {
        None => benchmark_average_speed(AMOUNT).await,
        Some(task) => {
            let task = usize::from_str(&task).unwrap();
            match task {
                1 => benchmark_average_speed(AMOUNT).await,
                2 => benchmark_average_speed_under_pressure(AMOUNT).await,
                3 => sleep_request_to_check_memory_in_task_manager(AMOUNT).await,
                _ => benchmark_average_speed(AMOUNT).await,
            }
        }
    }
    Ok(())
}

async fn sleep_request_to_check_memory_in_task_manager(amount: usize) {
    let now = Instant::now();
    let (sender, receiver) =  bounded(amount);
    for _ in 0..amount {
        sleep_request_spawn(sender.clone());
    }
    while !receiver.is_full() {
        sleep(Duration::from_millis(1)).await;
    }
    calculate_average(receiver).await;
    println!("Time fired in: {:#?}", now.elapsed());
}

async fn benchmark_average_speed_under_pressure(amount: usize) {
    let now = Instant::now();
    let (sender, receiver) =  bounded(amount);
    for _ in 0..amount {
        benchmark_speed_spawn(sender.clone());
    }
    while !receiver.is_full() {
        sleep(Duration::from_millis(1)).await;
    }
    calculate_average(receiver).await;
    println!("Time fired in: {:#?}", now.elapsed());
}

async fn benchmark_average_speed(amount: usize) {
    let now = Instant::now();
    let (sender, receiver) =  bounded(amount);
    for _ in 0..amount {
        benchmark_speed_spawn(sender.clone());
        sleep(Duration::from_millis(1)).await;
    }
    while !receiver.is_full() {
        sleep(Duration::from_millis(1)).await;
    }
    calculate_average(receiver).await;
    println!("Time fired in: {:#?}", now.elapsed());
}

fn benchmark_speed_spawn(sender: Sender<Duration>) -> JoinHandle<()> {
    async_std::task::spawn(benchmark_speed_future(sender))
}

async fn benchmark_speed_future(mut sender: Sender<Duration>) {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let now = Instant::now();
    stream.write_all(ROOT_REQUEST.as_ref()).await.unwrap();
    let buf = &mut [0u8; 4096];
    stream.read(buf).await.unwrap();
    sender.send(now.elapsed()).await.unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
}

fn sleep_request_spawn(sender: Sender<Duration>) -> JoinHandle<()> {
    async_std::task::spawn(sleep_request_future(sender))
}

async fn sleep_request_future(mut sender: Sender<Duration>) {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let now = Instant::now();
    stream.write_all(SLEEP_REQUEST.as_ref()).await.unwrap();
    let buf = &mut [0u8; 4096];
    stream.read(buf).await.unwrap();
    sender.send(now.elapsed()).await.unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
}

async fn calculate_average(mut receiver: Receiver<Duration>) {
    let mut total = 0u128;
    let mut amount = 0u128;
    while let Ok(duration) = receiver.try_recv() {
        println!("{:#?}", duration);
        amount+=1;
        total += duration.as_nanos();
    }
    let average = total / amount;

    println!("Avg: {:#?}", Duration::from_nanos(average as u64));
}