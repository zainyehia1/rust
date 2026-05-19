/* 
 * Because threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run.
   This can lead to problems, such as:
 * Race conditions, in which threads are accessing data or resources in an inconsistent order
 * Deadlocks, in which two threads are waiting for each other, preventing both threads from continuing
 * Bugs that only happen in certain situations and are hard to reproduce and fix reliably
 */

 use std::thread;
 use std::time::Duration;

fn main() {
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {i} from the spawned thread!");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    // The calls to `thread::sleep` force a thread to stop its execution for a short duration, allowing a different thread to run.
    
    // handle.join().unwrap();
    // Calling `join` on the handle blocks the thread currently running until the thread represented by the handle terminates.
    // Blocking a thread means that thread is prevented from performing work or exiting.
    
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
