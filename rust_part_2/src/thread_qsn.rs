/* Print the sum of 1-10**8 */
use std:: {
    sync::mpsc,
    thread::{self, spawn},
};
fn main() {
    let (tx, rx) = mpsc::channel();
    
    for i in 0..10 {

        //one such producer from multiple producers (mp)
        let producer = tx.clone();
        //move- moving the producer to the thread
        thread::spawn(move || {
            let mut sum: u64 = 0;
            for j in 0..10000000 {
                sum = sum + (i*1000000 + j);
            }
            producer.send(sum).unwrap();
        });
    }
    //the cloned producers are finished but the tx above the scope is always valid and the reciever keeps waiting for the 1st transmitter to go out of scope, so we exclusively drop the transmitter
    drop(tx);
    let mut sum: u64 = 0;
    //iterating over every value that is being recieved from transmitter
    for val in rx {
        sum = sum + val;
        println!("recieving value from all the 10 threads");
    }
    println!("Ans is {}", sum);
}