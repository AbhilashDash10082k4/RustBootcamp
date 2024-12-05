//various elems of a struct are stored in stack and heap based on their type
struct User {
    name: String,
    age: u32, //unsigned 32 bit integer
    active: bool,
}
fn main() {
    let name = String::from("AdSiNcE2k4");
    let user = User {
        name: name,
        age: 20,
        active: true,
    };
    println!("{} is {} years old ", user.name, user.age);
}
//tuple struct, unit struct