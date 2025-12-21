use minigrep::{search, search_case_insensitive};
use std::env;
use std::{error::Error, fs};

/*Config stores owned vals which transfers the ownership of config vars from args to the Config struct which violates ownership rules. So ,clone the args elems. This is inefficient but simple as there is no lifetime specification in this syntax

        //this creates an instance of Config ->so associate this with the struct and rename parse_config to new to justify the purpose

        //fn name changed to build as this is not a perfect fn and returns an error

        //return type - Result<Config, &static str> to return better error types

        /*chk for any env val set to switch the cases
        is_ok() -chks whether the val is set or not. If not set -sends false else true
        env::var sends Err variant if variable is set else Ok variant*/

*/
pub struct Config {
    pub query: String,
    pub file_path: String,
    //if case sensitive search is reqd or not
    pub switch_case: bool,
}
impl Config {
    //a default impl of Config
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            //better error type than panic
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let switch_case = env::var("SWITCH_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            switch_case,
        })
    }
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        //.expect("Should be able to read an existing file") -> this panicks
        let contents = fs::read_to_string(&self.file_path)?;

        let res = if self.switch_case {
            search(&self.query, &contents)
        } else {
            search_case_insensitive(&self.query, &contents)
        };
        for line in res {
            println!("With text: \n {}", line);
        }

        Ok(())
    }
}
