



pub fn analyse_log(word:&mut String, separating_strings: &mut Vec<String>, dictionary: &mut Vec<String>) {
    let mut total_levenstein = 0;
    print!("Analysing log {}", word);
    remove_request_type_from_log(word); // we exclude GET, POST, etc.
    let mut split_logs_to_check: Vec<String> = string_utils::split_string_by_multiple_delimiters(word, separating_strings);
        
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
            //println!("{} -> dict[{}]:{} = {}", split_logs_to_check[j], p, dictionary[p], min_levenstein);
            total_levenstein = total_levenstein + min_levenstein;
    }
    println!("Total levenstein: {}\n\n", total_levenstein);
}


pub fn analyse_logs(logs_to_check:&mut Vec<String>, separating_strings: &mut Vec<String>, dictionary: &mut Vec<String>) {
    for i in 0..logs_to_check.len() {
        analyse_log(&mut logs_to_check[i], separating_strings, dictionary); 
        if i == 30 {
            break;
        }
       // println!("\nTotal levenstein distance for log: {} is: {}\n\n\n", logs_to_check[i], total_levenstein);
    }
}

pub fn analyse_logs2(logs_to_check:&mut Vec<String>,start_index:usize, separating_strings: &mut Vec<String>, dictionary: &mut Vec<String>) {
    for i in start_index..logs_to_check.len() {
        analyse_log(&mut logs_to_check[i], separating_strings, dictionary); 
        if i == 30 {
            break;
        }
       // println!("\nTotal levenstein distance for log: {} is: {}\n\n\n", logs_to_check[i], total_levenstein);
    }
}

pub fn remove_request_type_from_log(log:&mut String) {
    let mut temp: Vec<String> = log.split(" ").map(|s| s.to_string()).collect();
    temp.remove(0);
    *log = temp.join(" ");
}