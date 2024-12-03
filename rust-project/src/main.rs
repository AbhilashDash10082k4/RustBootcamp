fn main() {
    println!("Hello, world!");

    //integers -signed an unsigned
    let mut x: i32 = -1000; //mut = mutable => can be changed
    let a: u32 = 1000;
    let b: u128 = 10000000000000000;
    let f: f32 = 7.777777;

    println!("x: {}, a: {}, b: {}, f: {}",x,a,b,f);

    //static checking
    let is_mard: bool = true;
    let is_successfull: bool = false;

    if is_mard {
        println!("tum ek mard ho");
    } else {
        println!("tum ek na mard ho");
    }
    if is_mard && is_successfull {
        println!("u won in life");
    } else {
        println!("Work hard!!");
    }

    let name: String = String::from("adsince2k4");
    println!("{}", name);
    //printing a specific char of the string name
    let char1 = name.chars().nth(0);
    
    println!("{}", char1.unwrap()); //Optionally a character so not able to print it, so unwrap is used

    let title = "abhi"; //by default takes the type as &str
    println!("{}", title);

}
