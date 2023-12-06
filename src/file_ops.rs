
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
// file_ops.rs

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;


/// Reads the entire contents of a file into a String.
pub fn read_file(file_name: &str) -> io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Reads a file line by line and returns a vector of strings.
pub fn read_file_line_by_line(file_name: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn load_files_into_vector(v:&mut Vec<String>, args: Vec<String>) {
    println!("Loading files {:?}", args);
    for filename in args {
        v.append(&mut read_file_line_by_line(filename.to_string()));
    }
}

pub fn load_files_into_vector2(v:&mut Vec<String>, args: &mut Vec<String>) {
    println!("Loading files {:?}", args);
    for filename in args {
        v.append(&mut read_file_line_by_line(filename.to_string()));
    }
}

/// Writes a vector of strings to a file, each string on a new line.
pub fn export_vector_to_file(v: &[String], file_name: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;
    for line in v {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

/// Checks if a file exists.
pub fn file_exists(file_name: &str) -> bool {
    Path::new(file_name).exists()
}

pub fn get_filenames_that_start_with(filename_start:String) -> Vec<String> {
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

// save string in a file in separate folder
pub fn save_string_in_file(string_to_save: String, file_name: String) {
    //handle error result of create_dir_all
    let res = fs::create_dir_all("hashes");
    let mut file = File::create("hashes/".to_owned() + &file_name).unwrap();
    file.write_all(string_to_save.as_bytes()).unwrap();
}

//read string from a file in seperate folder
pub fn read_string_from_file(file_name: String) -> String {
    let mut file = File::open("hashes/".to_owned() +&file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

//function to read a file starting at specific line
pub fn read_file_from_specific_line(file_name: String, line_number: usize) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut lines = Vec::new();
    let mut counter = 0;
    for i in 0..line_number {
        reader.read_line(&mut line).unwrap();
    }
    line.clear();
    //read until the end of the file
    while reader.read_line(&mut line).unwrap() != 0 {
        lines.push(line.clone());
        line.clear();
    }

    return lines;
}

//function that calculates number of lines in a file
pub fn calculate_number_of_lines_in_file(file_name: String) -> usize {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut counter = 0;
    //read until the end of the file
    while reader.read_line(&mut line).unwrap() != 0 {
        counter += 1;
        line.clear();
    }
    return counter;
}

/// Appends a vector of strings to a file, each string on a new line.
pub fn append_to_file(v: &[String], file_name: &str) -> io::Result<()> {
    let mut file = File::options().append(true).open(file_name)?;
    for line in v {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

/// Creates a directory if it does not exist.
pub fn create_dir_if_not_exists(dir: &str) -> io::Result<()> {
    if !Path::new(dir).exists() {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}

// ... other file operation functions as needed ...