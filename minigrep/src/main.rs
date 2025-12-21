use std::{env, process};
mod config;
use crate::config::{configure::Config};

/* Collecting user arguments in a vec-
Reference -chapter 12 I/O Project
let command:Vec<String> = env::args().collect();
dbg!(&command);
//env::args() = iterator, .collect() -turns the iterator into a collection

dbg- used with #[derive(Debug)] -used to print the types which do not implement Display trait by default and Debug(a trait) is used to print the all the values of the type -used with {:?} or {:#?} in println!()

dbg = macro, Debug = trait

prints the line no. and the val where the dbg! is used and takes ownership of the val (println! takes reference) and returns it back to the expression*, else completely takes the ownership of the variable passed into it
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    /*unwrap_or_else -> non panicking way to handle errors -
    if Ok(T) -> o/p = T
    if op = Err => closure will run
    */
    let config = Config::build(&args).unwrap_or_else(|err| {
        //eprintln -prints err to standard error stream
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //reading the file -
    if let Err(e) = config.run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
