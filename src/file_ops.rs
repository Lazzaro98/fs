/// File: file_ops.rs
///
/// This file contains functions for various file operations including reading, writing, and manipulating files.
/// These functions facilitate handling file I/O in a structured and reusable manner. The file includes the following functions:
///
/// - `read_file`: Reads the entire contents of a file into a String.
/// - `read_file_line_by_line`: Reads a file line by line and returns a vector of strings.
/// - `load_files_into_vector`: Loads multiple files into a vector of strings.
/// - `load_files_into_vector_ref`: Loads multiple files into a vector of strings (alternative version).
/// - `export_vector_to_file`: Writes a vector of strings to a file, each string on a new line.
/// - `file_exists`: Checks if a file exists.
/// - `get_filenames_with_prefix`: Retrieves filenames in a directory that start with a specified prefix.
/// - `save_string_in_file`: Saves a string in a file within a specific folder.
/// - `read_string_from_file`: Reads a string from a file in a specific folder.
/// - `read_file_from_specific_line`: Reads a file starting at a specific line.
/// - `calculate_number_of_lines`: Calculates the number of lines in a file.
/// - `append_to_file`: Appends a vector of strings to a file, each string on a new line.
/// - `create_dir_if_not_exists`: Creates a directory if it does not exist.
///
/// Author: Lazar Marinkovic
/// Date: July 7th, 2024


use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;
const DIR_PATH: &str = "C:\\Users\\Lazar\\Desktop\\Projects\\fs\\files";
/// Reads the entire contents of a file into a String.
///
/// # Parameters
/// - `file_name`: A reference to a string slice holding the name of the file.
///
/// # Returns
/// A result containing the file contents as a String or an error.
pub fn read_file(file_name: &str) -> io::Result<String>
{
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Reads a file line by line and returns a vector of strings.
///
/// # Parameters
/// - `file_name`: A reference to a string slice holding the name of the file.
///
/// # Returns
/// A result containing a vector of strings, each representing a line in the file, or an error.
pub fn read_file_line_by_line(file_name: &str) -> io::Result<Vec<String>>
{
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

/// Loads multiple files into a vector of strings.
///
/// # Parameters
/// - `v`: A mutable reference to a vector of strings.
/// - `file_names`: A vector of filenames to load.
///
/// # Returns
/// A result indicating success or failure.
pub fn load_files_into_vector(vector: &mut Vec<String>, file_names: Vec<String>) -> io::Result<()>
{
    for file_name in file_names
    {
        let mut lines = read_file_line_by_line(&file_name)?;
        vector.append(&mut lines);
    }
    Ok(())
}

/// Loads multiple files into a vector of strings (alternative version).
///
/// # Parameters
/// - `v`: A mutable reference to a vector of strings.
/// - `file_names`: A reference to a vector of filenames to load.
///
/// # Returns
/// A result indicating success or failure.
pub fn load_files_into_vector_ref(v: &mut Vec<String>, file_names: &Vec<String>) -> io::Result<()>
{
    for file_name in file_names
    {
        let mut lines = read_file_line_by_line(file_name)?;
        v.append(&mut lines);
    }
    Ok(())
}

/// Writes a vector of strings to a file, each string on a new line.
///
/// # Parameters
/// - `v`: A slice of strings to write.
/// - `file_name`: A reference to a string slice holding the name of the file.
///
/// # Returns
/// A result indicating success or failure.
pub fn export_vector_to_file(v: &[String], file_name: &str) -> io::Result<()>
{
    let mut file = File::create(file_name)?;
    for line in v
    {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

/// Checks if a file exists.
///
/// # Parameters
/// - `file_name`: A reference to a string slice holding the name of the file.
///
/// # Returns
/// A boolean indicating whether the file exists.
pub fn file_exists(file_name: &str) -> bool
{
    Path::new(file_name).exists()
}

/// Retrieves filenames in a directory that start with a specified prefix.
///
/// # Parameters
/// - `filename_prefix`: A string holding the prefix of filenames to retrieve.
///
/// # Returns
/// A vector of filenames.
pub fn get_filenames_with_prefix(filename_prefix: String) -> Vec<String>
{
    let mut filenames: Vec<String> = Vec::new();
    
    if let Err(e) = std::env::set_current_dir(DIR_PATH)
    {
        eprintln!("Failed to set current directory: {}", e);
    }
    
    for entry in fs::read_dir(".").unwrap()
    {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        
        if filename.starts_with(&filename_prefix)
        {
            filenames.push(filename.to_string());
        }
    }
    filenames
}

/// Saves a string in a file within a specific folder.
///
/// # Parameters
/// - `string_to_save`: A string to save.
/// - `file_name`: A string holding the name of the file to save to.
pub fn save_string_in_file(string_to_save: String, file_name: String)
{
    const DIR_PATH: &str = "hashes";
    
    fs::create_dir_all(DIR_PATH).unwrap();
    let mut file = File::create(format!("{}/{}", DIR_PATH, file_name)).unwrap();
    file.write_all(string_to_save.as_bytes()).unwrap();
}

/// Reads a string from a file in a specific folder.
///
/// # Parameters
/// - `file_name`: A string holding the name of the file to read from.
///
/// # Returns
/// The contents of the file as a string.
pub fn read_string_from_file(file_name: String) -> String
{
    let mut file = File::open(format!("hashes/{}", file_name)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

/// Reads a file starting at a specific line.
///
/// # Parameters
/// - `file_name`: A string holding the name of the file to read from.
/// - `line_number`: The line number to start reading from.
///
/// # Returns
/// A vector of strings from the specified line to the end of the file.
pub fn read_file_from_specific_line(file_name: String, line_number: usize) -> Vec<String>
{
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut line = String::new();

    for _ in 0..line_number
    {
        reader.read_line(&mut line).unwrap();
    }
    
    line.clear();
    while reader.read_line(&mut line).unwrap() != 0
    {
        lines.push(line.clone());
        line.clear();
    }
    
    lines
}

/// Calculates the number of lines in a file.
///
/// # Parameters
/// - `file_name`: A string holding the name of the file.
///
/// # Returns
/// The number of lines in the file.
pub fn calculate_number_of_lines(file_name: String) -> usize
{
    let file = match File::open(&file_name)
    {
        Ok(file) => file,
        Err(err) =>
        {
            println!("Failed to open file {}: {}", file_name, err);
            return 0;
        }
    };
    
    let reader = BufReader::new(file);
    reader.lines().count()
}

/// Appends a vector of strings to a file, each string on a new line.
///
/// # Parameters
/// - `v`: A slice of strings to append.
/// - `file_name`: A reference to a string slice holding the name of the file.
///
/// # Returns
/// A result indicating success or failure.
pub fn append_to_file(v: &[String], file_name: &str) -> io::Result<()>
{
    let mut file = File::options().append(true).open(file_name)?;
    for line in v
    {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

/// Creates a directory if it does not exist.
///
/// # Parameters
/// - `dir`: A reference to a string slice holding the name of the directory.
///
/// # Returns
/// A result indicating success or failure.
pub fn create_dir_if_not_exists(dir: &str) -> io::Result<()>
{
    if !Path::new(dir).exists()
    {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}
