/* */

//fn that takes a string as an input and returns the first word

fn main() {
    let line = String::from("Abhilash is a senior rust developer");

    let str_sliced = first_word(line);
    println!("{}", str_sliced);

    //a fn should be there where it looks into a part of string and not create a copy of the part of the string
    //this is string slice
    let word = String::from("AdSiNcE2k4");
    let word2 = &word[0..7]; //word2 is a reference of a part of word
    println!("{}", word2);

    //fn for str slice
    let word_3 = String::from("AbHiLaSh Dash");
    
    //in this fn the word is borrowed, the word_3_slice only refers to a certain portion of word and does not copy the part of word_3
    let word_3_slice = string_slice_first_word(&word_3);
    println!("{}", word_3_slice);
    
}

fn first_word(str: String)-> String {
    let mut ans = String::from("");
    for char in str.chars() {
        if char == ' ' {
            break;
        }
        ans.push_str(&char.to_string());
    }
    return ans;
    //this ans creates a copy of a part of the original string which takes more of the memory
}

//fn for string slice
fn string_slice_first_word(str: &String)->&str {
    let mut index = 0;
    for (_, i) in str.chars().enumerate() {
        if i == ' ' {
            break;
        }
        index = index + 1;
    }
    //slicing using index
    return &str[0..index];
}