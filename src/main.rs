
mod file_ops;
mod thread_utils;
mod log_ops;
mod string_utils;

use std::env;

fn main() {

    //read program parameters
    let args: Vec<String> = env::args().collect();

    let malicious_logs_filenames:Vec<String> = file_ops::get_filenames_that_start_with("malicious_logs".to_string());
    /*let mut malicious_counter: HashMap<String, usize> = HashMap::new();

    let mut malicious_logs: Vec<String> = Vec::new();
    let mut separating_strings: Vec<String> = Vec::new();
    let mut logs_to_check: Vec<String> = Vec::new();
    file_ops::load_files_into_vector2(&mut malicious_logs, &mut malicious_logs_filenames);
    file_ops::load_files_into_vector(&mut separating_strings, file_ops::get_filenames_that_start_with("special_strings".to_string()));
    file_ops::load_files_into_vector(&mut logs_to_check, file_ops::get_filenames_that_start_with("logs_to_check".to_string()));
    

   
    
    thread_utils::thread_that_waits_for_new_logs("logs_to_check.txt".to_string(), &mut logs_to_check, &mut separating_strings, &mut malicious_logs);

*/
}


