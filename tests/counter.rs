use parking_lot::{Mutex, RwLock};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::task::spawn;

#[tokio::test]
async fn lock_or_atomic() {
    let format_duration = |d: Duration| {
        let secs = d.as_secs();
        let millis = d.subsec_millis();
        let nanos = d.subsec_nanos() % 1_000_000;
        format!("{}s {}ms {}ns", secs, millis, nanos)
    };

    // 测试使用读写锁的性能
    let counter1 = Arc::new(RwLock::new(0i64));
    let start = std::time::Instant::now();
    let mut tasks = vec![];
    for _ in 0..100 {
        let counter = Arc::clone(&counter1);
        let task = spawn(async move {
            for _ in 0..10000 {
                let mut write_value = counter.write();
                *write_value += 1;
                drop(write_value);
                let read_value = counter.read();
                let _ = *read_value;
                drop(read_value);
            }
        });
        tasks.push(task);
    }
    futures::future::join_all(tasks).await;
    let elapsed = start.elapsed();
    let counter1_value = *counter1.read();

    // 测试使用互斥锁的性能
    let counter2 = Arc::new(Mutex::new(0i64));
    let start = std::time::Instant::now();
    let mut tasks = vec![];
    for _ in 0..100 {
        let counter = Arc::clone(&counter2);
        let task = spawn(async move {
            for _ in 0..10000 {
                {
                    let mut write_value = counter.lock();
                    *write_value += 1;
                    drop(write_value);
                    let read_value = counter.lock();
                    drop(read_value);
                }
            }
        });
        tasks.push(task);
    }
    futures::future::join_all(tasks).await;
    let elapsed2 = start.elapsed();
    let counter2_value = *counter2.lock();

    // 测试使用读写锁的性能
    let counter3 = Arc::new(RwLock::new(0i64));
    let start = std::time::Instant::now();
    let mut tasks = vec![];
    for _ in 0..100 {
        let counter = Arc::clone(&counter3);
        let task = spawn(async move {
            for _ in 0..10000 {
                let mut write_value = counter.write();
                *write_value += 1;
                drop(write_value);
                let read_value = counter.read();
                let _ = *read_value;
                drop(read_value);
            }
        });
        tasks.push(task);
    }
    futures::future::join_all(tasks).await;
    let elapsed3 = start.elapsed();
    let counter3_value = *counter3.read();

    // 测试不加锁的原子操作
    let counter4 = Arc::new(AtomicI64::new(0));
    let start = std::time::Instant::now();
    let mut tasks = vec![];
    for _ in 0..100 {
        let counter = Arc::clone(&counter4);
        let task = spawn(async move {
            for _ in 0..10000 {
                counter.fetch_add(1, Ordering::Relaxed);
                let _ = counter.load(Ordering::Relaxed);
            }
        });
        tasks.push(task);
    }
    futures::future::join_all(tasks).await;
    let elapsed4 = start.elapsed();
    let counter4_value = counter4.load(Ordering::Relaxed);
    // 输出结果
    println!("+----------------------+----------------------+----------------------+----------------------+");
    println!("| Lock type            | Counter value        | Elapsed time         | Throughput           |");
    println!("+----------------------+----------------------+----------------------+----------------------+");
    println!(
        "| Read-write lock      | {:<20} | {:<20} | {:<20} |",
        counter1_value,
        format_duration(elapsed),
        counter1_value as f64 / elapsed.as_secs_f64()
    );
    println!(
        "| Mutex                | {:<20} | {:<20} | {:<20} |",
        counter2_value,
        format_duration(elapsed2),
        counter2_value as f64 / elapsed2.as_secs_f64()
    );
    println!(
        "| Atomic with RwLock   | {:<20} | {:<20} | {:<20} |",
        counter3_value,
        format_duration(elapsed3),
        counter3_value as f64 / elapsed3.as_secs_f64()
    );
    println!(
        "| Atomic (no lock)     | {:<20} | {:<20} | {:<20} |",
        counter4_value,
        format_duration(elapsed4),
        counter4_value as f64 / elapsed4.as_secs_f64()
    );
    println!("+----------------------+----------------------+----------------------+----------------------+");
}
