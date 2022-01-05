fn get_char(string:String , i:usize) -> char {
    return string.chars().nth(i).unwrap();
}

fn is_url_encoded(three_consecutive_chars:String){
    // every url_encoded character's format is like %xy where xy is hexadecimal value
    
}

fn main() {
    let test:String = "Hello world".to_string();
    is_url_encoded(test);
}