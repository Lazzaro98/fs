use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn get_char(string: String, i: usize) -> char {
    return string.chars().nth(i).unwrap();
}

fn substr(str: String, pos: usize, len: i32) -> String {
    let ss: String = str.chars().skip(pos).take(len as usize).collect();
    return ss;
}

fn isSubstring(str:&mut String, sub:String) -> bool {
    for i in 0..str.len() {
        if substr(str.to_string(), i, sub.len() as i32) == sub {
            return true;
        }
    }
    return false;
}

// operation = 0 -> decode, operation = 1 -> remove
fn process_decoded_string(src: String, operation: i32) -> String {
    let mut ret: String = "".to_string();
    let mut i = 0;
    while i < src.chars().count() {
        if get_char(src.to_string(), i) == '%' {
            if operation == 0 {
                // ako decodujemo, onda lepimo dekodovan znak na rezultujuci strings. U suportnom samo preskocimo
                let t1: char = get_char(src.to_string(), i + 1);
                let t2: char = get_char(src.to_string(), i + 2);
                let a = t1 as u8 - 48; // oduzmemo vrednost '0'
                let b = t2 as u8 - 48;
                let decoded: char = ((a << 4) | b) as char;
                ret.push(decoded);
            }
            i = i + 3;
        } else {
            ret.push(get_char(src.to_string(), i));
            i = i + 1;
        }
    }
    return ret;
}

fn url_decode(src: String) -> String {
    // decodes URL encoded parts of the string
    return process_decoded_string(src, 0);
}

fn url_remove(src: String) -> String {
    // remove URL encoded parts of the string
    return process_decoded_string(src, 1);
}

fn tokenize_string_by_ngram(src: String, ngram: i32) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut _len = src.chars().count();
    while i + ngram <= _len as i32 {
        tokens.push(substr(src.to_string(), i as usize, ngram));
        i = i + 1;
    }
    return tokens;
}

fn extract_trigrams(src: String) -> Vec<String> {
    return tokenize_string_by_ngram(src, 3);
}

fn extract_bigrams(src: String) -> Vec<String> {
    return tokenize_string_by_ngram(src, 2);
}

fn extract_unigrams(src: String) -> Vec<String> {
    return tokenize_string_by_ngram(src, 1);
}

//function to replace string in string
fn replace_string_in_string(src: String, search: String, replace: String) -> String {
    let mut ret = "".to_string();
    let mut i = 0;
    while i < src.chars().count() {
        if substr(src.to_string(), i, search.chars().count() as i32) == search {
            ret = ret + &replace;
            i = i + search.chars().count();
        } else {
            ret = ret + &get_char(src.to_string(), i).to_string();
            i = i + 1;
        }
    }
    return ret;
}

//tokenize string by string
fn tokenize_string_by_string(src: String, delimiter: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut _len = src.chars().count();
    let mut curr_token = "".to_string();
    while i < _len {
        if substr(src.to_string(), i, delimiter.chars().count() as i32) == delimiter {
            tokens.push(curr_token);
            curr_token = "".to_string();
            i = i + delimiter.chars().count();
        } else {
            curr_token = curr_token + &get_char(src.to_string(), i).to_string();
            i = i + 1;
        }
     
    }
    if curr_token != "" {
        tokens.push(curr_token);
    }   
    return tokens;
}

//function to check if there is a substring of array in string
fn check_if_substring_in_array(src:&mut String, array: &mut Vec<String>) -> i32 {
    for i in 0..array.len() {
        if isSubstring(src, array[i].to_string()) {
            return i as i32;
        }
    }
    return -1;
}

//function to split string by multiple delimiters
fn split_string_by_multiple_delimiters(src:&mut String, delimiters:&mut Vec<String>) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut _len = src.chars().count();
    let mut curr_token = "".to_string();
    while i < _len {
        let mut found = false;
        for j in 0..delimiters.len() {
            if substr(src.to_string(), i, delimiters[j].chars().count() as i32) == delimiters[j] {
                tokens.push(curr_token);
                curr_token = "".to_string();
                found = true;
                i = i + delimiters[j].chars().count();
                break;
            }
        }
        if !found {
            curr_token = curr_token + &get_char(src.to_string(), i).to_string();
            i = i + 1;
        }
    }
    if curr_token != "" {
        tokens.push(curr_token);
    }   
    return tokens;
}

fn levenshtein(s1: &str, s2: &str) -> usize {
    if s1 == s2 {
        return 0;
    }

    if s1.chars().count() == 0 {
        return s2.chars().count();
    }

    if s2.chars().count() == 0 {
        return s1.chars().count();
    }

    let mut array: Vec<usize> = (1..).take(s1.chars().count()).collect();
    let mut dist_s1;
    let mut dist_s2;
    let mut ret = 0;
    for (index_s2, char_s2) in s2.chars().enumerate() {
        ret = index_s2;
        dist_s1 = index_s2;

        for (index_s1, char_s1) in s1.chars().enumerate() {
            if char_s1 == char_s2 {
                dist_s2 = dist_s1;
            } else {
                dist_s2 = dist_s1 + 1;
            }

            dist_s1 = array[index_s1];

            if dist_s1 > ret {
                if dist_s2 > ret {
                    ret = ret + 1;
                } else {
                    ret = dist_s2;
                }
            } else if dist_s2 > dist_s1 {
                ret = dist_s1 + 1;
            } else {
                ret = dist_s2;
            }

            array[index_s1] = ret;
        }
    }
    return ret;
}

fn dice_coefficient(s1: &str, s2: &str) -> f64 {
    let s1_length = s1.chars().count();
    let s2_length = s2.chars().count();

    if s1_length == 0 || s2_length == 0 {
        return 0.0;
    }

    if s1.eq(s2) {
        return 1.0;
    }

    let mut matches = 0;
    let mut i = 0;
    let mut j = 0;

    while i < s1_length && j < s2_length {
        let a: String = substr(s1.to_string(), i, 2);
        let b: String = substr(s2.to_string(), j, 2);
        let b_slice: &str = &b;
        if a.eq(b_slice) {
            matches = matches + 2;
        }
        i = i + 1;
        j = j + 1;
    }

    return (matches as f64) / (s1_length + s2_length) as f64;
}

//function to read file
fn read_file(file_name: String) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

//function to read file line by line and return a vector of strings
fn read_file_line_by_line(file_name: String) -> Vec<String> {
    
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents.lines().map(|s| s.to_string()).collect();
}

fn load_files_into_vector(v:&mut Vec<String>, args: Vec<String>) {
    for filename in args {
        v.append(&mut read_file_line_by_line(filename.to_string()));
    }
}

fn analyse_log(word:&mut String, separating_strings: &mut Vec<String>, dictionary: &mut Vec<String>) {
    let mut total_levenstein = 0;
    remove_request_type_from_log(word); // we exclude GET, POST, etc.
    let mut split_logs_to_check: Vec<String> = split_string_by_multiple_delimiters(word, separating_strings);
        
    for j in 0..split_logs_to_check.len() {
            let mut min_levenstein = 100;
            let mut p = 0;
            for k in 0..dictionary.len() {
                let levenstein = levenshtein(&split_logs_to_check[j], &dictionary[k]);
                if levenstein < min_levenstein {
                    min_levenstein = levenstein;
                    p = k;
                }
            }
            println!("{} -> dict[{}]:{} = {}", split_logs_to_check[j], p, dictionary[p], min_levenstein);
            total_levenstein = total_levenstein + min_levenstein;
    }
    println!("\nTotal levenstein: {}\n\n", total_levenstein);
}


fn analyse_logs(logs_to_check:&mut Vec<String>, separating_strings: &mut Vec<String>, dictionary: &mut Vec<String>) {
    for i in 0..logs_to_check.len() {
        analyse_log(&mut logs_to_check[i], separating_strings, dictionary); 
        if i == 30 {
            break;
        }
       // println!("\nTotal levenstein distance for log: {} is: {}\n\n\n", logs_to_check[i], total_levenstein);
    }
}

fn extract_dictionaries_from_logs(dictionary1:&mut Vec<String>, separating_strings: &mut Vec<String>) ->Vec<String>{
    let mut dictionary:Vec<String> = Vec::new();
    for i in 0..dictionary1.len() {
        let mut log = dictionary1[i].to_string();
        remove_request_type_from_log(&mut log); // we exclude GET, POST, etc.
        let mut temp: Vec<String> = split_string_by_multiple_delimiters(&mut log, separating_strings);
        for j in 0..temp.len() {
            //add temp[j] to dictionary only if it doesn't exist already
            if !dictionary.contains(&temp[j]) && temp[j] != "" {
                dictionary.push(temp[j].to_string());
            }
        }
      
    }
    return dictionary;
}

fn remove_request_type_from_log(log:&mut String) {
    let mut temp: Vec<String> = log.split(" ").map(|s| s.to_string()).collect();
    temp.remove(0);
    *log = temp.join(" ");
}

//function to export vector of strings to a file
fn export_vector_to_file(v: &Vec<String>, file_name: String) {
    let mut file = File::create(file_name).unwrap();
    for i in 0..v.len() {
        file.write_all(v[i].as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    }
}

//function to check if file exists
fn file_exists(file_name: String) -> bool {
    let path = Path::new(&file_name);
    return path.exists();
}

fn make_dictionary(dictionary1:&mut Vec<String>, separating_strings:&mut Vec<String>, update_dictionary:bool, file_name: String) -> Vec<String>{
    let mut dictionary:Vec<String> = Vec::new();
    if update_dictionary || !file_exists(file_name.to_string()) {
        dictionary = extract_dictionaries_from_logs(dictionary1, separating_strings);
        export_vector_to_file(&dictionary, file_name.to_string());
    }
    else {
        load_files_into_vector(&mut dictionary, vec![file_name.to_string()]);
    }
    return dictionary;
}

fn get_filenames_that_start_with(filename_start:String) -> Vec<String> {
    let mut malicious_logs_filenames: Vec<String> = Vec::new();
    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        if filename.starts_with(&filename_start) {
            malicious_logs_filenames.push(filename.to_string());
        }
    }
    return malicious_logs_filenames;
}

fn calculate_hash2<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

//function to calculate hash of a file
fn calculate_hash(file_name: String) -> String {
    let mut val = calculate_hash2(&file_name);
    let mut hash = String::new();
    return val.to_string();  
}

// save string in a file in separate folder
fn save_string_in_file(string_to_save: String, file_name: String) {
    //handle error result of create_dir_all
    let res = fs::create_dir_all("hashes");
    let mut file = File::create("hashes/".to_owned() + &file_name).unwrap();
    file.write_all(string_to_save.as_bytes()).unwrap();
}

//read string from a file in seperate folder
fn read_string_from_file(file_name: String) -> String {
    let mut file = File::open("hashes/".to_owned() +&file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

fn check_if_dictionaries_updated(filename_begin:String) -> bool {
    let mut malicious_logs_filenames = get_filenames_that_start_with(filename_begin);
    let mut hashes: Vec<String> = Vec::new();
    let mut hashes_from_file: Vec<String> = Vec::new();
    for i in 0..malicious_logs_filenames.len() {
        hashes.push(calculate_hash(malicious_logs_filenames[i].to_string()));
    }
    for i in 0..malicious_logs_filenames.len() {
        if file_exists("/hashes/".to_owned() + &malicious_logs_filenames[i]) {
            hashes_from_file.push(read_string_from_file(malicious_logs_filenames[i].to_string()));
        }
        else {
            return true;
        }
    }
    for i in 0..hashes.len() {
        if hashes[i] != hashes_from_file[i] {
            return true;
        }
    }
    return false;
}

// caluclates hash of given files and save them
fn calculate_and_save_hashes(filename_begin:String) {
    let mut malicious_logs_filenames = get_filenames_that_start_with(filename_begin);
    let mut hashes: Vec<String> = Vec::new();
    for i in 0..malicious_logs_filenames.len() {
        hashes.push(calculate_hash(malicious_logs_filenames[i].to_string()));
    }
    for i in 0..malicious_logs_filenames.len() {
        save_string_in_file(hashes[i].to_string(), malicious_logs_filenames[i].to_string());
    }
}


fn main() {

    //initiate boolean type var
    
    let mut request:String = "GET /api/areu/v1/housenumber?muni=Chrysos&town=Chrysos&street=Quanderious%20Friederich&cyr=true&fields=house_number,town_name,muni_name,street_name".to_string();
    // let url_decoded_request: String = url_remove(request);
    // println!("Decoded URL: {}", url_decoded_request);

    //let URL: String = "http://www.mysite.com/a%20file%20with%20spaces.html".to_string();
    //println!("\n\n\n");

    //println!("Test primer URL-a: {}\n", URL);
    //println!("Dekodovan URL: {}\n", url_decode(URL));
    //println!("URL sa uklonjenim URL-dekode karakterima: {}\n\n", url_remove(URL));
    
    //println!("Bigrami: {:?}", extract_unigrams(URL));
    //println!("Bigrami: {:?}", extract_bigrams(URL));
    //println!("Bigrami: {:?}", extract_trigrams(URL));
  
    /*println!("\n\n\n");
    println!("Test primer: {} i {}\n", str, str2);
    println!("Dice koeficijent slicnosti je: {:?}", dice_coefficient(&str, &str2));
    println!("Levenshtein koeficijent slicnosti je: {:?}", levenshtein_distance(&str, &str2));*/

    
    //println!("{:?}", read_file_line_by_line("test_file.txt".to_string()));

    //println!("{:?}", tokenize_string_by_special_character("test1_test2_test3_".to_string(), '_'));
    //println!("{:?}", tokenize_string_by_string("______".to_string(), "__".to_string()));
    

    let mut update_dictionary = true;
    
    let mut malicious_logs: Vec<String> = Vec::new();
    let mut separating_strings: Vec<String> = Vec::new();
    let mut logs_to_check: Vec<String> = Vec::new();
    load_files_into_vector(&mut malicious_logs, get_filenames_that_start_with("malicious_logs".to_string()));
    load_files_into_vector(&mut separating_strings, get_filenames_that_start_with("special_strings".to_string()));
    load_files_into_vector(&mut logs_to_check, get_filenames_that_start_with("logs_to_check".to_string()));

    //get all filenames that start with "malicious_logs"
   
    update_dictionary = check_if_dictionaries_updated("malicious_logs".to_string());
    let mut dictionary:Vec<String> = make_dictionary(&mut malicious_logs, &mut separating_strings, update_dictionary, "dictionary.txt".to_string());

    if update_dictionary {
        calculate_and_save_hashes("malicious_logs".to_string());
    }

    analyse_logs(&mut logs_to_check, &mut separating_strings, &mut dictionary);
    
    //save_string_in_file("test_file".to_string(), "test_file".to_string());
}


