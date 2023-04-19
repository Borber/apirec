use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use parking_lot::{Mutex, RwLock};

#[test] 
fn lock_or_atomic() {
    let format_duration = |d: Duration| {
        let secs = d.as_secs();
        let millis = d.subsec_millis();
        let nanos = d.subsec_nanos() % 1_000_000;
        format!("{}s {}ms {}ns", secs, millis, nanos)
    };

    // 测试使用读写锁的性能
    let counter1 = Arc::new(RwLock::new(0i64));
    let start = Instant::now();
    let mut threads = vec![];
    for _ in 0..100 {
        let counter = Arc::clone(&counter1);
        let thread = thread::spawn(move || {
            for _ in 0..10000 {
                let mut write_value = counter.write();
                *write_value += 1;
                drop(write_value);
                let read_value = counter.read();
                let _ = *read_value;
                drop(read_value);
            }
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let elapsed = start.elapsed();
    let counter1_value = *counter1.read();

    // 测试使用互斥锁的性能
    let counter2 = Arc::new(Mutex::new(0i64));
    let start = Instant::now();
    let mut threads = vec![];
    for _ in 0..100 {
        let counter = Arc::clone(&counter2);
        let thread = thread::spawn(move || {
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
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let elapsed2 = start.elapsed();
    let counter2_value = *counter2.lock();

    // 测试使用原子操作的性能
    let counter3 = Arc::new(RwLock::new(AtomicI64::new(0)));
    let start = Instant::now();
    let mut threads = vec![];
    for _ in 0..100 {
        let counter = Arc::clone(&counter3);
        let thread = thread::spawn(move || {
            for _ in 0..10000 {
                counter.write().fetch_add(1, Ordering::SeqCst);
                let _ = counter.read().load(Ordering::SeqCst);
            }
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let elapsed3 = start.elapsed();
    let counter3_value = counter3.read().load(Ordering::SeqCst);

    // 测试不加锁的原子操作
    let counter4 = Arc::new(AtomicI64::new(0));
    let start = Instant::now();
    let mut threads = vec![];
    for _ in 0..100 {
        let counter = Arc::clone(&counter4);
        let thread = thread::spawn(move || {
            for _ in 0..10000 {
                counter.fetch_add(1, Ordering::SeqCst);
                let _ = counter.load(Ordering::SeqCst);
            }
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let elapsed4 = start.elapsed();
    let counter4_value = counter4.load(Ordering::SeqCst);

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
