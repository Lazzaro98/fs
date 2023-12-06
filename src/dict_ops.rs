
mod log_ops;
mod string_utils;
mod file_ops;

pub fn extract_dictionaries_from_logs(dictionary1:&mut Vec<String>, separating_strings: &mut Vec<String>) ->Vec<String>{
    println!("Extracting dictionaries from logs...");
    let mut dictionary:Vec<String> = Vec::new();
    for i in 0..dictionary1.len() {
        let mut log = dictionary1[i].to_string();
        log_ops::remove_request_type_from_log(&mut log); // we exclude GET, POST, etc.
        let mut temp: Vec<String> = string_utils::split_string_by_multiple_delimiters(&mut log, separating_strings);
        for j in 0..temp.len() {
            //add temp[j] to dictionary only if it doesn't exist already
            if !dictionary.contains(&temp[j]) && temp[j] != "" {
                dictionary.push(temp[j].to_string());
            }
        }
      
    }
    return dictionary;
}

pub fn make_dictionary(dictionary1:&mut Vec<String>, separating_strings:&mut Vec<String>, update_dictionary:bool, file_name: String) -> Vec<String>{
    let mut dictionary:Vec<String> = Vec::new();
    if update_dictionary || !file_ops::file_exists(file_name.to_string()) {
        println!("Malicious files had been updated. Making dictionary...");
        dictionary = log_ops::extract_dictionaries_from_logs(dictionary1, separating_strings);
        file_ops::export_vector_to_file(&dictionary, file_name.to_string());
    }
    else {
        println!("Loading existing dictionary...");
        file_ops::load_files_into_vector(&mut dictionary, vec![file_name.to_string()]);
    }
    return dictionary;
}

pub fn check_if_dictionaries_updated(filename_begin:String) -> bool {
    let mut malicious_logs_filenames = file_ops::get_filenames_that_start_with(filename_begin);
    let mut hashes: Vec<String> = Vec::new();
    let mut hashes_from_file: Vec<String> = Vec::new();
    for i in 0..malicious_logs_filenames.len() {
        hashes.push(hash_ops::calculate_hash(malicious_logs_filenames[i].to_string()));
    }

    for i in 0..malicious_logs_filenames.len() {
        if file_ops::file_exists("hashes/".to_owned() + &malicious_logs_filenames[i]) {
            hashes_from_file.push(file_ops::read_string_from_file(malicious_logs_filenames[i].to_string()));
        }
        else {
            return true;
        }
    }

    for i in 0..hashes.len() {
        println!("{} comparing with {}", hashes[i], hashes_from_file[i]);
        if hashes[i] != hashes_from_file[i] {
            return true;
        }
    }
    return false;
}