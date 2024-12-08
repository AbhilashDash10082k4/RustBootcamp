use std::fmt::Display;

//structs and lifetimes
struct User <'a>{
    name: &'a str,
}

fn main() {
    let ans;
    let str1 = String::from("abhilash");
    {
        let str2 = String::from("dash");
        ans = longest_str(&str1, &str2);
    }
    /*the str2 is scoped out so the string gets cleared from the heap

    the str2 lifetime ends within th escope and the ans is pointing towards the str2 due to which after the scope it becomes a dangling pointer

    the ans was valid previously when it had the ownership of str2 but when it is borrowing the str2 its becomes a dangling pointer after the scope*/
    // println!("{}", ans);

    //structs with lifetime
    let name = String::from("abhilash");
    let user = User {
        name: &name,
    };
    println!("{}", user.name);
}

/* 'a tells that str1 and str2 and the return value are related, the lifetime of return type id the intersection of lifetimes of inputs*/
fn longest_str<'a>(str1: &'a str, str2: &'a str) -> &'a str{
    if str1.len() > str2.len() {
        return str1;
    } else {
        return str2;
    }
}