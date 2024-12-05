/*run time errors- final binary was created but error came while running the binary file

compilation error- the binary file was not created due to type checker, borrow checker complaining

*/
//result enum
//dynamic type allocation

use std::fs;

struct Point<T, U> {
    x: T,
    y: T,
    z: U,
}
enum Result<A, B> {
    Ok(A),
    Err(B),
}

fn main() {
    let int_point = Point {x: 1, y: 2, z:"1"};
    let str_point = Point {x: "1", y: "2",z: 3};
    let float_point = Point {x: 1.0, y: 2.0, z:3.6};
    println!("Integer points: {}, {}", int_point.x, int_point.y);
    println!("Str points: {}, {}", str_point.x, str_point.y);
    println!("Float points: {}, {}", float_point.x, float_point.y);

    //erroneous line
    //reading file
    let res = fs::read_to_string("example.txt");

    //error catching
    match res {
        Ok(content) => {
            println!("File content {}", content);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
    let res_1 = read_from_file("example.txt".to_string());
    println!("hi error");

    //Option enum
    let my_str_option_enum = String::from("Abhilash");
    let res = first_a(my_str_option_enum);
    match res {
        Some(index) => println!("The letter a is found at index {}", index),
        None => println!("The letter is not found in the string"),
    }
}

fn read_from_file(file_content: String) -> String {
    let res = fs::read_to_string("xyz.txt");
    return res.unwrap(); //res.unwrap() will either return string or wil crash the code
}

//Option enum-  return an option instead of returning a null value
fn first_a(s: String)-> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}