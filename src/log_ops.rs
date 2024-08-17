/// File: log_analysis.rs
///
/// This file contains functions for analyzing log entries by calculating Levenshtein distances and Dice coefficients.
/// These functions are used to compare log entries against a dictionary of known patterns.
/// The file includes the following functions:
///
/// - `analyze_log_and_determine_criticality`: Analyzes a single log entry and calculates its total Levenshtein distance and Dice coefficient, then determines the criticality of the request.
/// - `analyze_logs`: Analyzes multiple log entries from the beginning with an optional limit.
/// - `analyze_logs_from_index`: Analyzes multiple log entries starting from a specified index with an optional limit.
/// - `remove_request_type_from_log`: Removes the request type (e.g., GET, POST) from a log entry.
///
/// Modules Required:
/// - `string_utils`: Provides string manipulation utilities including Levenshtein distance and Dice coefficient calculation.
///
/// Author: Lazar Marinkovic
/// Date: July 7th, 2024

use crate::string_utils;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Determines the criticality of a request based on Levenshtein distance and Dice coefficient.
///
/// # Parameters
/// - `levenshtein_distance`: The Levenshtein distance between the request and the dictionary entry.
/// - `dice_coefficient`: The Dice coefficient between the request and the dictionary entry.
/// - `max_levenshtein_distance`: The maximum possible Levenshtein distance (e.g., the length of the longest string).
///
/// # Returns
/// A string indicating the criticality level (Low, Medium, High).
pub fn determine_criticality(levenshtein_distance: usize, dice_coefficient: f64, max_levenshtein_distance: usize) -> String
{
    // Normalize Levenshtein distance
    let normalized_levenshtein = 1.0 - (levenshtein_distance as f64 / max_levenshtein_distance as f64);

    // Weights for composite score (these can be adjusted based on your preference)
    let alpha = 0.5;
    let beta = 0.5;

    // Calculate composite score
    let composite_score = alpha * normalized_levenshtein + beta * dice_coefficient;

    // Determine criticality based on composite score
    if composite_score < 0.3 {
        "Low".to_string()
    } else if composite_score < 0.7 {
        "Medium".to_string()
    } else {
        "High".to_string()
    }
}

/// Analyzes a single log entry and calculates its total Levenshtein distance and Dice coefficient,
/// then determines the criticality of the request.
///
/// # Parameters
/// - `log_entry`: A mutable reference to the log entry string to be analyzed.
/// - `delimiters`: A mutable reference to a vector of delimiter strings used for splitting the log entry.
/// - `dictionary`: A mutable reference to a vector of known patterns to compare against.
/// - `max_levenshtein_distance`: The maximum possible Levenshtein distance.
pub fn analyze_log_and_determine_criticality(log_entry: &mut String, delimiters: &mut Vec<String>, dictionary: &mut Vec<String>, max_levenshtein_distance: usize)
{
    let mut total_levenshtein = 0;
    let mut total_dice_coefficient = 0.0;
    let mut num_entries = 0;

    print!("Analyzing log {}", log_entry);
    remove_request_type_from_log(log_entry); // Exclude request types like GET, POST, etc.
    let split_log_entries: Vec<String> = string_utils::split_by_multiple_delimiters(log_entry, delimiters);

    for entry in split_log_entries.iter()
    {
        let mut min_levenshtein = usize::MAX;
        let mut max_dice_coefficient = 0.0;

        for pattern in dictionary.iter()
        {
            let levenshtein_distance = string_utils::levenshtein(entry, pattern);
            let dice_coefficient = string_utils::dice_coefficient(entry, pattern);

            if levenshtein_distance < min_levenshtein
            {
                min_levenshtein = levenshtein_distance;
            }

            if dice_coefficient > max_dice_coefficient
            {
                max_dice_coefficient = dice_coefficient;
            }
        }

        total_levenshtein += min_levenshtein;
        total_dice_coefficient += max_dice_coefficient;
        num_entries += 1;
    }

    // Average the scores
    if num_entries > 0 {
        total_levenshtein /= num_entries;
        total_dice_coefficient /= num_entries as f64;
    }

    println!("Total Levenshtein distance: {}", total_levenshtein);
    println!("Total Dice coefficient: {}", total_dice_coefficient);

    let criticality = determine_criticality(total_levenshtein, total_dice_coefficient, max_levenshtein_distance);
    println!("Request Criticality: {}\n\n", criticality);
}

/// Analyzes multiple log entries from the beginning with an optional limit.
///
/// # Parameters
/// - `logs`: A mutable reference to a vector of log entry strings to be analyzed.
/// - `delimiters`: A mutable reference to a vector of delimiter strings used for splitting log entries.
/// - `dictionary`: A mutable reference to a vector of known patterns to compare against.
/// - `limit`: An optional limit on the number of log entries to analyze.
/// - `max_levenshtein_distance`: The maximum possible Levenshtein distance.
pub fn analyze_logs(logs: &mut Vec<String>, delimiters: &mut Vec<String>, dictionary: &mut Vec<String>, limit: Option<usize>, max_levenshtein_distance: usize)
{
    let max_entries = limit.unwrap_or(logs.len());
    for (index, log) in logs.iter_mut().enumerate().take(max_entries)
    {
        analyze_log_and_determine_criticality(log, delimiters, dictionary, max_levenshtein_distance);
    }
}

/// Analyzes multiple log entries starting from a specified index with an optional limit.
///
/// # Parameters
/// - `logs`: A mutable reference to a vector of log entry strings to be analyzed.
/// - `start_index`: The index from which to start analyzing log entries.
/// - `delimiters`: A mutable reference to a vector of delimiter strings used for splitting log entries.
/// - `dictionary`: A mutable reference to a vector of known patterns to compare against.
/// - `limit`: An optional limit on the number of log entries to analyze.
/// - `max_levenshtein_distance`: The maximum possible Levenshtein distance.
pub fn analyze_logs_from_index(logs: &mut Vec<String>, start_index: usize, delimiters: &mut Vec<String>, dictionary: &mut Vec<String>, limit: Option<usize>, max_levenshtein_distance: usize)
{
    let max_entries = limit.unwrap_or(logs.len() - start_index) + start_index;
    for index in start_index..max_entries
    {
        if index < logs.len()
        {
            analyze_log_and_determine_criticality(&mut logs[index], delimiters, dictionary, max_levenshtein_distance);
        }
        else
        {
            break;
        }
    }
}

/// Removes the request type (e.g., GET, POST) from a log entry.
///
/// # Parameters
/// - `log_entry`: A mutable reference to the log entry string.
pub fn remove_request_type_from_log(log_entry: &mut String)
{
    let mut parts: Vec<String> = log_entry.split(" ").map(|s| s.to_string()).collect();
    parts.remove(0);
    *log_entry = parts.join(" ");
}
