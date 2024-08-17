/// File: string_utils.rs
///
/// This file contains functions for various string manipulation utilities including substring extraction,
/// tokenization, replacement, and similarity calculations such as Levenshtein distance and Dice coefficient.
/// The file includes the following functions:
///
/// - `get_char_at`: Returns a character at a given index from a string.
/// - `substring`: Returns a substring from a given string, starting at a specified position with a specified length.
/// - `contains_substring`: Checks if a substring exists within a string.
/// - `tokenize_by_ngram`: Tokenizes a string into n-grams.
/// - `extract_trigrams`: Extracts trigrams from a string.
/// - `extract_bigrams`: Extracts bigrams from a string.
/// - `extract_unigrams`: Extracts unigrams from a string.
/// - `replace_substring`: Replaces a specified substring within a string with another substring.
/// - `tokenize_by_delimiter`: Tokenizes a string by a delimiter string.
/// - `contains_any_substring`: Checks if any substring in an array exists in a given string.
/// - `split_by_multiple_delimiters`: Splits a string by multiple delimiters.
/// - `process_decoded_string`: Processes a URL encoded string for decoding or removal of encoded parts.
/// - `url_decode`: Decodes URL encoded parts of the string.
/// - `url_remove`: Removes URL encoded parts of the string.
/// - `levenshtein`: Calculates the Levenshtein distance between two strings.
/// - `dice_coefficient`: Calculates the Dice coefficient between two strings.
///
/// Author: Lazar Marinkovic
/// Date: July 7th, 2024

/// Returns a character at a given index from a string.
///
/// # Parameters
/// - `string`: A string from which to get the character.
/// - `index`: The index of the character to retrieve.
///
/// # Returns
/// The character at the specified index.
pub fn get_char_at(string: &str, index: usize) -> char
{
    string.chars().nth(index).unwrap()
}

/// Returns a substring from a given string, starting at a specified position with a specified length.
///
/// # Parameters
/// - `string`: The input string.
/// - `start`: The starting position of the substring.
/// - `length`: The length of the substring.
///
/// # Returns
/// The substring.
pub fn substring(string: &str, start: usize, length: usize) -> String
{
    string.chars().skip(start).take(length).collect()
}

/// Checks if a substring exists within a string.
///
/// # Parameters
/// - `string`: The input string.
/// - `substring`: The substring to check for.
///
/// # Returns
/// A boolean indicating whether the substring exists within the string.
pub fn contains_substring(string: &str, substring: &str) -> bool
{
    string.contains(substring)
}

/// Tokenizes a string into n-grams.
///
/// # Parameters
/// - `string`: The input string.
/// - `ngram_size`: The size of the n-grams.
///
/// # Returns
/// A vector of n-grams.
pub fn tokenize_by_ngram(string: &str, ngram_size: usize) -> Vec<String>
{
    let mut tokens = Vec::new();
    let length = string.chars().count();

    if ngram_size > length
    {
        return tokens;
    }

    for i in 0..=length - ngram_size
    {
        tokens.push(substring(string, i, ngram_size));
    }
    
    tokens
}

/// Extracts trigrams from a string.
///
/// # Parameters
/// - `string`: The input string.
///
/// # Returns
/// A vector of trigrams.
pub fn extract_trigrams(string: &str) -> Vec<String>
{
    tokenize_by_ngram(string, 3)
}

/// Extracts bigrams from a string.
///
/// # Parameters
/// - `string`: The input string.
///
/// # Returns
/// A vector of bigrams.
pub fn extract_bigrams(string: &str) -> Vec<String>
{
    tokenize_by_ngram(string, 2)
}

/// Extracts unigrams from a string.
///
/// # Parameters
/// - `string`: The input string.
///
/// # Returns
/// A vector of unigrams.
pub fn extract_unigrams(string: &str) -> Vec<String>
{
    tokenize_by_ngram(string, 1)
}

/// Replaces a specified substring within a string with another substring.
///
/// # Parameters
/// - `source`: The source string.
/// - `search`: The substring to search for.
/// - `replace`: The substring to replace the search string with.
///
/// # Returns
/// The resulting string after replacement.
pub fn replace_substring(source: &str, search: &str, replace: &str) -> String
{
    source.replace(search, replace)
}

/// Tokenizes a string by a delimiter string.
///
/// # Parameters
/// - `source`: The source string.
/// - `delimiter`: The delimiter string.
///
/// # Returns
/// A vector of tokens.
pub fn tokenize_by_delimiter(source: &str, delimiter: &str) -> Vec<String>
{
    source.split(delimiter).map(|s| s.to_string()).collect()
}

/// Checks if any substring in an array exists in a given string.
///
/// # Parameters
/// - `source`: The source string.
/// - `substrings`: A vector of substrings to check for.
///
/// # Returns
/// The index of the found substring in the array, or -1 if not found.
pub fn contains_any_substring(source: &str, substrings: &[String]) -> i32
{
    for (index, substring) in substrings.iter().enumerate()
    {
        if contains_substring(source, substring)
        {
            return index as i32;
        }
    }
    
    -1
}

/// Splits a string by multiple delimiters.
///
/// # Parameters
/// - `source`: The source string.
/// - `delimiters`: A vector of delimiter strings.
///
/// # Returns
/// A vector of tokens.
pub fn split_by_multiple_delimiters(source: &str, delimiters: &[String]) -> Vec<String>
{
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let length = source.chars().count();
    let mut i = 0;

    while i < length
    {
        let mut matched = false;

        for delimiter in delimiters
        {
            if source[i..].starts_with(delimiter)
            {
                if !current_token.is_empty()
                {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }

                i += delimiter.chars().count();
                matched = true;
                break;
            }
        }

        if !matched
        {
            current_token.push(get_char_at(source, i));
            i += 1;
        }
    }

    if !current_token.is_empty()
    {
        tokens.push(current_token);
    }

    tokens
}

/// Processes a URL encoded string for decoding or removal of encoded parts.
///
/// # Parameters
/// - `source`: The source string.
/// - `operation`: The operation to perform (0 for decode, 1 for remove).
///
/// # Returns
/// The processed string.
pub fn process_decoded_string(source: &str, operation: i32) -> String
{
    let mut result = String::new();
    let length = source.chars().count();
    let mut i = 0;

    while i < length
    {
        if get_char_at(source, i) == '%'
        {
            if operation == 0
            {
                let decoded_char = u8::from_str_radix(&source[i+1..i+3], 16).unwrap() as char;
                result.push(decoded_char);
            }
            
            i += 3;
        }
        else
        {
            result.push(get_char_at(source, i));
            i += 1;
        }
    }

    result
}

/// Decodes URL encoded parts of the string.
///
/// # Parameters
/// - `source`: The source string.
///
/// # Returns
/// The decoded string.
pub fn url_decode(source: &str) -> String
{
    process_decoded_string(source, 0)
}

/// Removes URL encoded parts of the string.
///
/// # Parameters
/// - `source`: The source string.
///
/// # Returns
/// The string with URL encoded parts removed.
pub fn url_remove(source: &str) -> String
{
    process_decoded_string(source, 1)
}

/// Calculates the Levenshtein distance between two strings.
///
/// # Parameters
/// - `s1`: The first string.
/// - `s2`: The second string.
///
/// # Returns
/// The Levenshtein distance.
pub fn levenshtein(s1: &str, s2: &str) -> usize
{
    if s1 == s2
    {
        return 0;
    }

    if s1.is_empty()
    {
        return s2.chars().count();
    }

    if s2.is_empty()
    {
        return s1.chars().count();
    }

    let mut prev_costs: Vec<usize> = (0..=s1.len()).collect();
    let mut curr_costs = vec![0; s1.len() + 1];

    for (i, s2_char) in s2.chars().enumerate()
    {
        curr_costs[0] = i + 1;

        for (j, s1_char) in s1.chars().enumerate()
        {
            let cost = if s1_char == s2_char { 0 } else { 1 };
            curr_costs[j + 1] = *[
                curr_costs[j] + 1,
                prev_costs[j + 1] + 1,
                prev_costs[j] + cost,
            ].iter().min().unwrap();
        }

        prev_costs.clone_from_slice(&curr_costs);
    }

    curr_costs[s1.len()]
}

/// Calculates the Dice coefficient between two strings.
///
/// # Parameters
/// - `s1`: The first string.
/// - `s2`: The second string.
///
/// # Returns
/// The Dice coefficient.
pub fn dice_coefficient(s1: &str, s2: &str) -> f64
{
    let s1_bigrams = extract_bigrams(s1);
    let s2_bigrams = extract_bigrams(s2);

    let intersection_count = s1_bigrams.iter().filter(|&bigram| s2_bigrams.contains(bigram)).count();
    let total_bigrams = s1_bigrams.len() + s2_bigrams.len();

    (2 * intersection_count) as f64 / total_bigrams as f64
}
