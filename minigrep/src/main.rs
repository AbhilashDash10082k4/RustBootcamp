use minigrep::{search, search_case_insensitive};
use std::{env, error::Error, fs, process};
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
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //reading the file -
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //.expect("Should be able to read an existing file") -> this panicks
    let contents = fs::read_to_string(config.file_path)?;

    let res = if config.switch_case {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in res {
        println!("With text: \n {}", line);
    }

    Ok(())
}
pub struct Config {
    pub query: String,
    pub file_path: String,
    //if case sensitive search is reqd or not
    pub switch_case: bool,
}
impl Config {
    //a default impl of Config
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            //better error type than panic
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        //Config stores owned vals which transfers the ownership of config vars from args to the Config struct which violates ownership rules. So ,clone the args elems. This is inefficient but simple as there is no lifetime specification in this syntax

        //this creates an instance of Config ->so associate this with the struct and rename parse_config to new to justify the purpose

        //fn name changed to build as this is not a perfect fn and returns an error

        //return type - Result<Config, &static str> to return better error types

        /*chk for any env val set to switch the cases
        is_ok() -chks whether the val is set or not. If not set -sends false else true
        env::var sends Err variant if variable is set else Ok variant*/
        let switch_case = env::var("SWITCH_CASE").is_ok();
        Ok(Config { query, file_path, switch_case })
    }
}
