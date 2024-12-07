//finding bigger char adn integer
fn main() {
    let larger_int = largest_int(2, 7);
    let larger_char = largest_char('a', 'v');
    println!("{}", larger_int);
    println!("{}", larger_char);

    //using generics
    let largest_char = find_largest('a', 'x');
    let largest_int = find_largest(10, 90);
    println!("{}", largest_char);
    println!("{}", largest_int);

}
fn find_largest<T: std::cmp::PartialOrd>(a:T, b:T)-> T{
    if a > b {
        a
    }else {
        b
    }
}

fn largest_char(a: char, b:char)->char {
    if a > b{
        a
    }else {
        b
    }
}
fn largest_int(a: i32, b: i32)-> i32 {
    if a > b {
        a
    } else {
        b
    }
}