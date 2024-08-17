
//Import HashMap
use std::collections::HashMap;

mod file_ops;
mod thread_utils;
mod log_ops;
mod string_utils;

use std::env;

fn main() {

    //read program parameters
    let _args: Vec<String> = env::args().collect();

    let mut malicious_logs: Vec<String> = Vec::new();
    let mut separating_strings: Vec<String> = Vec::new();
    let mut logs_to_check: Vec<String> = Vec::new();

    // Loading malicious logs
    let mut _malicious_logs_filenames:Vec<String> = file_ops::get_filenames_with_prefix("malicious_logs".to_string());
    if let Err(e) = file_ops::load_files_into_vector_ref(&mut malicious_logs, &_malicious_logs_filenames)
    {
        eprintln!("Error loading malicious logs: {}", e);
    }
    
    // Loading seperating strings
    let separating_strings_filenames = file_ops::get_filenames_with_prefix("special_strings".to_string());
    if let Err(e) = file_ops::load_files_into_vector(&mut separating_strings, separating_strings_filenames)
    {
        eprintln!("Error loading separating strings: {}", e);
    }

    // Loading logs that need to be checked
    let logs_to_check_filenames = file_ops::get_filenames_with_prefix("logs_to_check".to_string());
    if let Err(e) = file_ops::load_files_into_vector(&mut logs_to_check, logs_to_check_filenames)
    {
        eprintln!("Error loading logs to check: {}", e);
    }

    // Starting a live thread, that will wait for new logs (changes in log files)
    thread_utils::watch_for_new_log_entries("logs_to_check.txt".to_string(), &mut logs_to_check, &mut separating_strings, &mut malicious_logs);
}


