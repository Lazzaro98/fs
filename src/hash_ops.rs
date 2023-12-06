
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

mod file_ops;

fn calculate_hash2<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

//function to calculate hash of a file
fn calculate_hash(file_name: String) -> String {
    //get content from a file
    let content = file_ops::read_file(file_name);
    let mut val = calculate_hash2(&content);
    let mut hash = String::new();
    return val.to_string();  
}

// caluclates hash of given files and save them
fn calculate_and_save_hashes(filename_begin:String) {
    let mut malicious_logs_filenames = file_ops::get_filenames_that_start_with(filename_begin);
    let mut hashes: Vec<String> = Vec::new();
    for i in 0..malicious_logs_filenames.len() {
        hashes.push(calculate_hash(malicious_logs_filenames[i].to_string()));
    }
    for i in 0..malicious_logs_filenames.len() {
        file_ops::save_string_in_file(hashes[i].to_string(), malicious_logs_filenames[i].to_string());
    }
}
