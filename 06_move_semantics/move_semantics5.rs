#![allow(clippy::ptr_arg)]


fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}


fn string_uppercase(data: String) {
    let data = data.to_uppercase(); 
    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);          
    string_uppercase(data);   
}
