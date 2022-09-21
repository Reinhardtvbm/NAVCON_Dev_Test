use std::io::stdin;

pub fn get_user_input(message: &str) -> char {
    println!("{}", message);

    let mut input = String::new();

    stdin()
    .read_line(&mut input)
    .ok()
    .expect("No Message to read");

    get_char(&input, 0)
}

pub fn get_char(string: &String, index: usize) -> char {
    string.as_str().chars().nth(index).expect("No character at that index")
}

pub fn get_char_str(str: &str, index: usize) -> char {
    str.chars().nth(index).expect("No character at that index")
}