//various elems of a struct are stored in stack and heap based on their type
struct User {
    name: String,
    age: u32, //unsigned 32 bit integer
    active: bool,
}
//implementing structs
struct Square {
    len: u32,
    breadth: u32,
}
impl Square {
    fn area(&self) -> u32 { //self is the 1st attribute
        return self.len*self.breadth;
    }
    //can have multiple implementations
    fn peri(&self) -> u32 {
        return 2*(self.len + self.breadth);
    }
}

fn main() {
    let name = String::from("AdSiNcE2k4");
    let user = User {
        name: name,
        age: 20,
        active: true,
    };
    println!("{} is {} years old ", user.name, user.age);

    let square = Square {
        len: 1,
        breadth: 7,
    };
    println!("The area is {}", square.area());
    println!("The perimeter is {}", square.peri());
}
//tuple struct, unit struct
