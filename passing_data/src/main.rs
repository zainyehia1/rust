use std::sync::mpsc; // multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel(); // mpsc cannot infer the type of the parameter `T` declared on the function channel
    // Moving tx to a spawned thread and sending "hi"
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap(); // val is moved here
    // }); // The spawned thread needs to own the transmitter to be able to send messages through the channel.

    let tx1 = tx.clone();
    thread::spawn(move || {
       let vals = vec![
           String::from("hi"),
           String::from("from"),
           String::from("the"),
           String::from("thread"),          
       ];

       for val in vals {
           tx1.send(val).unwrap();
           thread::sleep(Duration::from_millis(500));
       }
    });

    thread::spawn(move || {
       let vals = vec![
           String::from("more"),
           String::from("messages"),
           String::from("for"),
           String::from("you"),      
       ];

       for val in vals {
           tx.send(val).unwrap();
           thread::sleep(Duration::from_secs(1));
       }
    });
    
    for received in rx {
        println!("Got: {received}");
    }
}
