#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

static FLAG: AtomicUsize = AtomicUsize::new(0);
const TIME_QUANTUM: u64 = 10;


// I AM NOT DONE

fn spawned_thread() {
        println!("Spawned-thread is waiting ...");
        while FLAG.load(Ordering::Relaxed) < 1 {
            task_tick(); // For cooperative scheduler, we must yield here!
            // For preemptive scheduler, just relaxed! Leave it for scheduler.
        }

        let _ = FLAG.fetch_add(1, Ordering::Relaxed);
}

fn task_tick() {
    thread::sleep(Duration::from_millis(TIME_QUANTUM));
    thread::yield_now();
}
    // Give spawned thread a chance to start.

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let spawned_handle = thread::spawn(spawned_thread);

    thread::yield_now();

    println!("Main thread set FLAG to notify spawned-thread to continue.");

    let _ = FLAG.fetch_add(1, Ordering::Relaxed);

    thread::sleep(Duration::from_millis(10));

    println!("Main thread waits spawned-thread to respond ...");

    while FLAG.load(Ordering::Relaxed) < 2 {
        task_tick();
    }

    println!("Preempt test run OK!");

    println!("\n[ArceOS Tutorial]: A2 okay!");

    let _ = spawned_handle.join();
}