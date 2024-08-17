/// File: hash_ops.rs
///
/// This file contains functions for calculating hashes of various entities, including strings and files.
/// These hashes are used to identify and compare content, particularly for detecting changes or ensuring integrity.
/// The file includes the following functions:
///
/// - `calculate_hash_from_object`: Calculates a hash for a given input using the `DefaultHasher`.
/// - `calculate_file_hash`: Calculates the hash of a file's content.
/// - `calculate_and_save_file_hashes`: Calculates and saves the hashes of files that start with a given prefix.
///
/// Modules Required:
/// - `file_ops`: Provides file handling utilities.
///
/// Author: Lazar Marinkovic
/// Date: July 7th, 2024

use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
mod file_ops;

/// Calculates a hash for a given input using the `DefaultHasher`.
///
/// # Parameters
/// - `input`: A reference to the input to hash, which implements the `Hash` trait.
///
/// # Returns
/// The calculated hash as a `u64`.
pub fn calculate_hash_from_object<T: Hash>(input: &T) -> u64
{
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

/// Calculates the hash of a file's content.
///
/// # Parameters
/// - `file_name`: A string holding the name of the file.
///
/// # Returns
/// The calculated hash as a string.
pub fn calculate_file_hash(file_name: String) -> String
{
    let content = file_ops::read_file(&file_name).expect("Failed to read file");
    let hash_value = calculate_hash_from_object(&content);
    hash_value.to_string()
}

/// Calculates and saves the hashes of files that start with a given prefix.
///
/// # Parameters
/// - `filename_prefix`: A string representing the prefix of filenames to hash and save.
pub fn calculate_and_save_file_hashes(filename_prefix: String)
{
    let matching_filenames = file_ops::get_filenames_with_prefix(filename_prefix);
    let mut file_hashes: Vec<String> = Vec::new();
    
    for filename in &matching_filenames
    {
        file_hashes.push(calculate_file_hash(filename.clone()));
    }
    
    for (file_hash, filename) in file_hashes.iter().zip(matching_filenames.iter())
    {
        file_ops::save_string_in_file(file_hash.to_string(), filename.to_string());
    }
}
