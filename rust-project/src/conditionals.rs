fn main() {
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

    for i in 0..8 {
        println!("{}", i);
    }
    
    //iterating over str, arr, maps
    //printing the 1st word of the char
    let line:String = String::from("Aham Brahma Asmi");
    let first_word = get_first_word(line);
    println!("{}", first_word);
}

fn get_first_word(line: String)-> String {
    //logic to get the first word
    let mut word_1st:String = String::from("");

    for char in line.chars() {
        word_1st.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return word_1st;
}