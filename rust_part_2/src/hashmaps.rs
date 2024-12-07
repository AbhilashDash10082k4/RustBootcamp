/* key val pairs */
use std::collections::HashMap; //importing hashmaps

fn main() {
    let mut users: HashMap<String, u32> = HashMap::new();

    users.insert(String::from("Abhi"), 22);
    users.insert(String::from("adsince2k4"), 69);

    let users_age = users.get("Abhi");
    match users_age {
        Some(age) => println!("age is {}", age),
        None => println!("User not found"),
    }

    let vec_of_tuples = vec![(String::from("abhilash"), 21), (String::from("a senior Rust developer"), 69)];

    println!("{:?}", vec_of_tuples);

    let new_hash_map = get_vector_of_tuples(vec_of_tuples);

    println!("{:?}", new_hash_map);
}

//fn that takes a vector of tuples as input and returns a hashmap of string and no

fn get_vector_of_tuples(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hash_map = HashMap::new();
    for (key, val) in vec {
        hash_map.insert(key, val);
    }
    return hash_map;
}