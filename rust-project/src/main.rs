mod conditionals;
mod memory;
mod ownership;
mod structs;
mod borrowing_references;
mod error_handling;

mod enums;
fn main() {
    
    println!("Hello, world!");

    //integers -signed an unsigned
    let mut x: i32 = -1000; //mut = mutable => can be changed
    let a: u32 = 1000;
    let b: u128 = 10000000000000000;
    let f: f32 = 7.777777;

    println!("x: {}, a: {}, b: {}, f: {}",x,a,b,f);

    let name: String = String::from("adsince2k4");
    println!("{}", name);
    //printing a specific char of the string name
    let char1 = name.chars().nth(0);
    
    println!("{}", char1.unwrap()); //Optionally a character so not able to print it, so unwrap is used

    let title = "abhi"; //by default takes the type as &str
    println!("{}", title);

}
