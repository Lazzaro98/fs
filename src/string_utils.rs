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

// string_utils.rs

/// Returns a character at a given index from a string.
pub fn get_char(string: String, i: usize) -> char {
    string.chars().nth(i).unwrap()
}

/// Returns a substring from a given string, starting at a specified position with a specified length.
pub fn substr(str: String, pos: usize, len: i32) -> String {
    str.chars().skip(pos).take(len as usize).collect()
}

/// Checks if a substring exists within a string.
pub fn is_substring(str: &String, sub: String) -> bool {
    for i in 0..str.len() {
        if substr(str.to_string(), i, sub.len() as i32) == sub {
            return true;
        }
    }
    false
}

/// Tokenizes a string into n-grams.
pub fn tokenize_string_by_ngram(src: String, ngram: i32) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let len = src.chars().count();
    while i + ngram <= len as i32 {
        tokens.push(substr(src.to_string(), i as usize, ngram));
        i += 1;
    }
    tokens
}

/// Extracts trigrams from a string.
pub fn extract_trigrams(src: String) -> Vec<String> {
    tokenize_string_by_ngram(src, 3)
}

/// Extracts bigrams from a string.
pub fn extract_bigrams(src: String) -> Vec<String> {
    tokenize_string_by_ngram(src, 2)
}

/// Extracts unigrams from a string.
pub fn extract_unigrams(src: String) -> Vec<String> {
    tokenize_string_by_ngram(src, 1)
}

/// Replaces a specified substring within a string with another substring.
pub fn replace_string_in_string(src: String, search: String, replace: String) -> String {
    let mut ret = String::new();
    let mut i = 0;
    while i < src.chars().count() {
        if substr(src.to_string(), i, search.chars().count() as i32) == search {
            ret += &replace;
            i += search.chars().count();
        } else {
            ret += &get_char(src.to_string(), i).to_string();
            i += 1;
        }
    }
    ret
}

/// Tokenizes a string by a delimiter string.
pub fn tokenize_string_by_string(src: String, delimiter: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let len = src.chars().count();
    let mut curr_token = String::new();
    while i < len {
        if substr(src.to_string(), i, delimiter.chars().count() as i32) == delimiter {
            tokens.push(curr_token);
            curr_token = String::new();
            i += delimiter.chars().count();
        } else {
            curr_token += &get_char(src.to_string(), i).to_string();
            i += 1;
        }
    }
    if !curr_token.is_empty() {
        tokens.push(curr_token);
    }
    tokens
}

/// Checks if any substring in an array exists in a given string.
pub fn check_if_substring_in_array(src: &String, array: &Vec<String>) -> i32 {
    for (i, item) in array.iter().enumerate() {
        if is_substring(src, item.to_string()) {
            return i as i32;
        }
    }
    -1
}

/// Splits a string by multiple delimiters.
pub fn split_string_by_multiple_delimiters(src: &String, delimiters: &Vec<String>) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let len = src.chars().count();
    let mut curr_token = String::new();
    while i < len {
        let mut found = false;
        for delimiter in delimiters {
            if substr(src.to_string(), i, delimiter.chars().count() as i32) == *delimiter {
                tokens.push(curr_token);
                curr_token = String::new();
                found = true;
                i += delimiter.chars().count();
                break;
            }
        }
        if !found {
            curr_token += &get_char(src.to_string(), i).to_string();
            i += 1;
        }
    }
    if !curr_token.is_empty() {
        tokens.push(curr_token);
    }
    tokens
}

// operation = 0 -> decode, operation = 1 -> remove
pub fn process_decoded_string(src: String, operation: i32) -> String {
    let mut ret: String = "".to_string();
    let mut i = 0;
    while i < src.chars().count() {
        if get_char(src.to_string(), i) == '%' {
            if operation == 0 {
                // ako decodujemo, onda lepimo dekodovan znak na rezultujuci strings. U suportnom samo preskocimo
                let t1: char = get_char(src.to_string(), i + 1);
                let t2: char = get_char(src.to_string(), i + 2);
                let a = t1 as u8 - 48; // oduzmemo vrednost '0'
                let b = t2 as u8 - 48;
                let decoded: char = ((a << 4) | b) as char;
                ret.push(decoded);
            }
            i = i + 3;
        } else {
            ret.push(get_char(src.to_string(), i));
            i = i + 1;
        }
    }
    return ret;
}

pub fn url_decode(src: String) -> String {
    // decodes URL encoded parts of the string
    return process_decoded_string(src, 0);
}

pub fn url_remove(src: String) -> String {
    // remove URL encoded parts of the string
    return process_decoded_string(src, 1);
}

pub fn levenshtein(s1: &str, s2: &str) -> usize {
    if s1 == s2 {
        return 0;
    }

    if s1.chars().count() == 0 {
        return s2.chars().count();
    }

    if s2.chars().count() == 0 {
        return s1.chars().count();
    }

    let mut array: Vec<usize> = (1..).take(s1.chars().count()).collect();
    let mut dist_s1;
    let mut dist_s2;
    let mut ret = 0;
    for (index_s2, char_s2) in s2.chars().enumerate() {
        ret = index_s2;
        dist_s1 = index_s2;

        for (index_s1, char_s1) in s1.chars().enumerate() {
            if char_s1 == char_s2 {
                dist_s2 = dist_s1;
            } else {
                dist_s2 = dist_s1 + 1;
            }

            dist_s1 = array[index_s1];

            if dist_s1 > ret {
                if dist_s2 > ret {
                    ret = ret + 1;
                } else {
                    ret = dist_s2;
                }
            } else if dist_s2 > dist_s1 {
                ret = dist_s1 + 1;
            } else {
                ret = dist_s2;
            }

            array[index_s1] = ret;
        }
    }
    return ret;
}

pub fn dice_coefficient(s1: &str, s2: &str) -> f64 {
    let s1_length = s1.chars().count();
    let s2_length = s2.chars().count();

    if s1_length == 0 || s2_length == 0 {
        return 0.0;
    }

    if s1.eq(s2) {
        return 1.0;
    }

    let mut matches = 0;
    let mut i = 0;
    let mut j = 0;

    while i < s1_length && j < s2_length {
        let a: String = substr(s1.to_string(), i, 2);
        let b: String = substr(s2.to_string(), j, 2);
        let b_slice: &str = &b;
        if a.eq(b_slice) {
            matches = matches + 2;
        }
        i = i + 1;
        j = j + 1;
    }

    return (matches as f64) / (s1_length + s2_length) as f64;
}