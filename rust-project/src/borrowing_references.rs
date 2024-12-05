/*reference- giving address of a string rather than ownership of a string or a var

a var can have multiple borrowers and only 1 owner untill n unless the var is not doing hanky panky with the owner or borrower, doing hanky panky means getting changed or modified

multiple mutable references are not allowed to prevent memory issues also an immutable reference cannot be done if a var has already been refered mutably*/

fn main() {
    let mut my_string = String::from("adsince2k4");
    borrow(&my_string); //& reference
    mutably_borrow(&mut my_string);

    let mut my_str = String::from("abhi");
    mutably_borrow_update(&mut my_str);
}

fn borrow(some_str: &String) {
    println!("{}", some_str);
}

fn mutably_borrow(some_string: &mut String) {
    some_string.push_str(" is a senior rust developer");
    println!("{}", some_string);
}

fn mutably_borrow_update(some_str: &mut String) {
    some_str.push_str(" is a solana devloper");
    print!("{}", some_str);
}