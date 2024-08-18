use std::sync::mpsc;
extern crate notify;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::time::Duration;
use crate::file_ops;
use crate::log_ops;

/// Creates a thread that waits for changes in malicious log files.
///
/// # Parameters
/// - `filename_prefix`: A string representing the prefix of filenames to watch for changes.
/// - `malicious_logs`: A mutable reference to a vector to store the malicious log entries.
pub fn watch_for_malicious_log_changes(filename_prefix: String, malicious_logs: &mut Vec<String>)
{
    let (tx, rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
    watcher.watch(".", RecursiveMode::Recursive).unwrap();

    loop
    {
        match rx.recv()
        {
            Ok(event) =>
            {
                match event
                {
                    DebouncedEvent::Create(_) =>
                    {
                        println!("File created");
                    },
                    DebouncedEvent::Write(_) =>
                    {
                        malicious_logs.clear();
                        let malicious_log_files: Vec<String> = file_ops::get_filenames_with_prefix(filename_prefix.clone());
                        let _ = file_ops::load_files_into_vector(malicious_logs, malicious_log_files);
                        println!("Updated malicious logs");
                    },
                    _ => {}
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

/// Creates a thread that waits for new log entries and processes them.
///
/// # Parameters
/// - `filename`: A string representing the filename to watch for new log entries.
/// - `logs`: A mutable reference to a vector to store the log entries.
/// - `delimiters`: A mutable reference to a vector of delimiter strings used for splitting log entries.
/// - `dictionary`: A mutable reference to a vector of known patterns to compare against.
pub fn watch_for_new_log_entries(filename: String, logs: &mut Vec<String>, delimiters: &mut Vec<String>, dictionary: &mut Vec<String>)
{
    let (tx, rx) = mpsc::channel();
    let mut current_length = logs.len();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
    watcher.watch(".", RecursiveMode::Recursive).unwrap();

    loop
    {
        match rx.recv()
        {
            Ok(event) =>
            {
                match event
                {
                    DebouncedEvent::Create(_) =>
                    {
                        println!("File created");
                    },
                    DebouncedEvent::Write(_) =>
                    {
                        let new_length = file_ops::calculate_number_of_lines(filename.clone());

                        if current_length != new_length
                        {
                            println!("Processing new log entries in: {:?}", filename);

                            let mut new_logs: Vec<String> = file_ops::read_file_from_specific_line(filename.clone(), current_length);
                            logs.append(&mut new_logs);
                            log_ops::analyze_logs_from_index(logs, current_length, delimiters, dictionary, None);
                            current_length = new_length;
                        }
                    },
                    _ => {}
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
