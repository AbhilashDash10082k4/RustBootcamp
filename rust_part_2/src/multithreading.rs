/*Multithreading - using multiple cores*/
use std::thread;
use std::time::Duration;
use std::sync::mpsc; //mpsc -multiple producers single consumers

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..11 {
            println!("Hi from spawned thread {}", i);
        }
    });
    handle.join().unwrap();
    for i in 0..11 {
        println!("Hi from main thread {}", i);
    }

    //vec in thread
    let x = 1;
    {
        let vec = vec![1,2,3,4,5,6,7];
        //move keyword moves the ownership of vec to the closure (||)
        let handle_2 = thread::spawn(move || {
            println!("{:?}", vec);
        });
    }
    println!("{}", x);

    //message passing- creating chnnl and passing data form one thread to other thread, if one of th etransmitter or reciever goes out of the scope then the chnnl is said to be closed

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg_send = String::from("hii");
        //unwrap()-> panicks the thread, it doesnt catch the thread, it stops the execution of the code
        tx.send(msg_send).unwrap();
    });
    let recieved = rx.recv();
    let recieved_match = match recieved {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("{}", err),
    };
}