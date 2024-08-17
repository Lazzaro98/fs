/// File: dict_ops.rs
///
/// This file contains functions for processing log files to extract and manage dictionaries
/// of unique strings. These dictionaries are used to detect and compare log entries,
/// specifically for identifying malicious patterns. The file includes the following functions:
///
/// - `extract_unique_entries_from_logs`: Processes log entries to extract unique strings, excluding request types.
/// - `generate_or_load_dictionary`: Generates a dictionary from log entries or loads an existing dictionary from a file.
/// - `are_dictionaries_updated`: Compares the current hashes of log files with the stored hashes to determine if updates are needed.
///
/// Modules Required:
/// - `log_ops`: Provides operations to modify log entries.
/// - `string_utils`: Provides string manipulation utilities.
/// - `file_ops`: Provides file handling utilities.
/// - `hash_ops`: Provides hashing utilities.
///
/// Author: Lazar Marinkovic
/// Date: July 7th, 2024

mod log_ops;
mod string_utils;
mod file_ops;
mod hash_ops;

/// Extract unique dictionary entries from log entries
///
/// # Summary
/// Processes log entries to extract unique strings, excluding request types (e.g., GET, POST).
///
/// # Parameters
/// - `log_entries`: A mutable reference to a vector of log entry strings.
/// - `delimiters`: A mutable reference to a vector of delimiter strings used for splitting log entries.
///
/// # Returns
/// A vector of unique strings extracted from log entries.
pub fn extract_unique_entries_from_logs(log_entries: &mut Vec<String>, delimiters: &mut Vec<String>) -> Vec<String>
{
    println!("Extracting unique entries from logs...");
    let mut unique_entries: Vec<String> = Vec::new();
    
    for entry in log_entries.iter_mut()
    {
        log_ops::remove_request_type_from_log(entry); // Exclude request types like GET, POST, etc.
        let temp_entries = string_utils::split_string_by_multiple_delimiters(entry, delimiters);
        
        for temp_entry in temp_entries
        {
            if !unique_entries.contains(&temp_entry) && !temp_entry.is_empty()
            {
                unique_entries.push(temp_entry);
            }
        }
    }
    
    unique_entries
}

/// Generate or load a dictionary of log entries (ex make_dictionary)
///
/// # Summary
/// Generates a dictionary from log entries or loads an existing dictionary from a file.
/// Optionally updates the dictionary if specified or if the dictionary file doesn't exist.
///
/// # Parameters
/// - `log_entries`: A mutable reference to a vector of log entry strings.
/// - `delimiters`: A mutable reference to a vector of delimiter strings used for splitting log entries.
/// - `update_dictionary`: A boolean flag to force update of the dictionary.
/// - `file_name`: The name of the file to load or save the dictionary.
///
/// # Returns
/// A vector representing the dictionary of log entries.
pub fn generate_or_load_dictionary(log_entries: &mut Vec<String>, delimiters: &mut Vec<String>, update_dictionary: bool, file_name: String) -> Vec<String>
{
    let mut dictionary: Vec<String> = Vec::new();
    
    if update_dictionary || !file_ops::file_exists(&file_name)
    {
        println!("Updating malicious files dictionary...");
        dictionary = extract_unique_entries_from_logs(log_entries, delimiters);
        file_ops::export_vector_to_file(&dictionary, &file_name);
    }
    else
    {
        println!("Loading existing dictionary...");
        file_ops::load_files_into_vector(&mut dictionary, vec![file_name]);
    }
    
    dictionary
}

/// Check if the log dictionaries have been updated (ex check_if_dictionaries_updated)
///
/// # Summary
/// Compares the current hashes of log files with the stored hashes to determine if updates are needed.
///
/// # Parameters
/// - `filename_prefix`: A string representing the prefix of filenames to check.
///
/// # Returns
/// A boolean indicating whether the dictionaries have been updated.
pub fn are_dictionaries_updated(filename_prefix: String) -> bool
{
    let log_filenames = file_ops::get_filenames_with_prefix(filename_prefix);
    let mut current_hashes: Vec<String> = Vec::new();
    let mut stored_hashes: Vec<String> = Vec::new();
    
    for filename in &log_filenames
    {
        current_hashes.push(hash_ops::calculate_hash(filename.clone()));
    }

    for filename in &log_filenames
    {
        let hash_file = format!("hashes/{}", filename);
        if file_ops::file_exists(&hash_file)
        {
            stored_hashes.push(file_ops::read_string_from_file(hash_file));
        }
        else
        {
            return true;
        }
    }

    for (current_hash, stored_hash) in current_hashes.iter().zip(stored_hashes.iter())
    {
        println!("Comparing {} with {}", current_hash, stored_hash);
        if current_hash != stored_hash
        {
            return true;
        }
    }
    
    false
}
