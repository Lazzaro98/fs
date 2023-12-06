
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::mpsc;
use std::collections::HashMap;
use std::env;

extern crate notify;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::time::Duration;
use std::io::BufReader;
use std::io::BufRead;

// thread that will be waiting for file changes
pub fn thread_that_waits_for_malicious_log_changes(filename_begin:String, malicious_log:&mut Vec<String>) {
    let (tx, rx) = mpsc::channel();
    
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
    watcher.watch(".", RecursiveMode::Recursive).unwrap();
    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    DebouncedEvent::Create(_) => {
                        println!("File created");
                    },
                    DebouncedEvent::Write(smth) => {
                        //clear malicious_logs_filenames
                       
                        malicious_log.clear();
                        let mut malicious_logs_filenames:Vec<String> = file_ops::get_filenames_that_start_with(filename_begin.clone());

                        //load malicious logs into malicious_log
                        file_ops::load_files_into_vector(malicious_log, malicious_logs_filenames);
                        println!("Updated malicious logs");
                        
                    },
                    _ => {}
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

pub fn thread_that_waits_for_new_logs(filename:String, logs:&mut Vec<String>, separating_strings:&mut Vec<String>, dictionary:&mut Vec<String>) {
    let (tx, rx) = mpsc::channel();
    let mut length = logs.len();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
    watcher.watch(".", RecursiveMode::Recursive).unwrap();
    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    DebouncedEvent::Create(_) => {
                        println!("File created");
                    },
                    DebouncedEvent::Write(smth) => {
                        //length of logs before update
                        let new_length = file_ops::calculate_number_of_lines_in_file(filename.clone());

                        if length != new_length {
                            // read file from specific line length
                            let mut new_logs:Vec<String> = file_ops::read_file_from_specific_line(filename.clone(), length);
                            logs.append(&mut new_logs);
                            log_ops::analyse_logs2(logs, length, separating_strings, dictionary);
                            length = new_length;
                        }
                        
                       
                    },
                    _ => {}
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}