use std::env;
use std::fs;
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

    let config = Config::new(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    //reading the file -
    let contents =
        fs::read_to_string(config.file_path).expect("Should be able to read an existing file");
    println!("With text:\n{contents}");
}
struct Config {
    query: String,
    file_path: String,
}
impl Config {
    //a default impl of Config
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        //Config stores owned vals which transfers the ownership of config vars from args to the Config struct which violates ownership rules. So ,clone the args elems. This is inefficient but simple as there is no lifetime specification in this syntax

        //this creates an instance of Config ->so associate this with the struct and rename parse_config to new to justify the purpose
        Config { query, file_path }
    }
}
