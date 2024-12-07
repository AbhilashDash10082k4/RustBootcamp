// trait Summary {
//     //a blue print for structs
//     fn summarize(&self) ->String;
// }
// struct User {
//     name: String,
//     age: u32,
// }
// //self implementation
// impl Summary for User {
//     fn summarize(&self) -> String{
//         return format!("{} is {} years old", self.name, self.age);
//     }
// }

// //trait with default implementation
// struct User_1{
//     name: String,
//     age: u32,
// }
// trait DefaultImplementation {
//     fn default_summarize(&self) -> String {
//         return String::from("hi");
//     }
// }
// impl DefaultImplementation for User_1 {}

// fn main() {
//     let user = User {
//         name: String::from("Abhilash"),
//         age: 21,
//     };
//     println!("{}", user.summarize());

//     //default implementation
//     let user_1 = User_1 {
//         name: String::from("adsince2k4"),
//         age:21,
//     };
//     println!("{}", user_1.default_summarize());

// }

//traits as parameter
trait Summary {
    fn summarize(&self) -> String {
        return String::from("hi");
    }
}
trait Trait_Bound {
    fn trait_bound(&self) -> String {
        return String::from("hello");
    }
}
struct User {
    name: String,
    age: u32,
}
struct Fix;
impl Summary for Fix{}
impl Summary for String{}
impl Summary for User{}
impl Trait_Bound for Fix{}

fn main() {
    let user = User {
        name: String::from("Abhilash"),
        age: 21,
    };
    notify(user);
}
fn notify(u: impl Summary) {
    println!("{}", u.summarize());
}

//trait bound
fn notify_trait_bound<T: Summary + Trait_Bound>(item: T) {
    println!("{}", item.summarize());
}