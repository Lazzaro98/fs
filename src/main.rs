use std::fs::File;
use std::io::Read;

fn get_char(string: String, i: usize) -> char {
    return string.chars().nth(i).unwrap();
}

fn substr(str: String, pos: usize, len: i32) -> String {
    let ss: String = str.chars().skip(pos).take(len as usize).collect();
    return ss;
}

// operation = 0 -> decode, operation = 1 -> remove
fn process_decoded_string(src: String, operation: i32) -> String {
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

fn url_decode(src: String) -> String {
    // decodes URL encoded parts of the string
    return process_decoded_string(src, 0);
}

fn url_remove(src: String) -> String {
    // remove URL encoded parts of the string
    return process_decoded_string(src, 1);
}

fn tokenize_string_by_ngram(src: String, ngram: i32) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut _len = src.chars().count();
    while i + ngram <= _len as i32 {
        tokens.push(substr(src.to_string(), i as usize, ngram));
        i = i + 1;
    }
    return tokens;
}

fn extract_trigrams(src: String) -> Vec<String> {
    return tokenize_string_by_ngram(src, 3);
}

fn extract_bigrams(src: String) -> Vec<String> {
    return tokenize_string_by_ngram(src, 2);
}

fn extract_unigrams(src: String) -> Vec<String> {
    return tokenize_string_by_ngram(src, 1);
}


//split function
fn tokenize_string_by_special_character(src: String, delimiter: char) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut _len = src.chars().count();
    let mut curr_token = "".to_string();
    while i < _len {

        if get_char(src.to_string(), i) == delimiter {
            tokens.push(curr_token);
            curr_token = "".to_string();
        } else {
            curr_token = curr_token + &get_char(src.to_string(), i).to_string();
        }
        i = i + 1;
    }
    if curr_token != "" {
        tokens.push(curr_token);
    }   
    return tokens;
}

//function to replace string in string
fn replace_string_in_string(src: String, search: String, replace: String) -> String {
    let mut ret = "".to_string();
    let mut i = 0;
    while i < src.chars().count() {
        if substr(src.to_string(), i, search.chars().count() as i32) == search {
            ret = ret + &replace;
            i = i + search.chars().count();
        } else {
            ret = ret + &get_char(src.to_string(), i).to_string();
            i = i + 1;
        }
    }
    return ret;
}

//tokenize string by string
fn tokenize_string_by_string(src: String, delimiter: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let mut _len = src.chars().count();
    let mut curr_token = "".to_string();
    while i < _len {

        if substr(src.to_string(), i, delimiter.chars().count() as i32) == delimiter {
            tokens.push(curr_token);
            curr_token = "".to_string();
            i = i + delimiter.chars().count();
        } else {
            curr_token = curr_token + &get_char(src.to_string(), i).to_string();
            i = i + 1;
        }
     
    }
    if curr_token != "" {
        tokens.push(curr_token);
    }   
    return tokens;
}

fn levenshtein(s1: &str, s2: &str) -> usize {
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

fn dice_coefficient(s1: &str, s2: &str) -> f64 {
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

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_length = s1.chars().count();
    let s2_length = s2.chars().count();

    if s1_length == 0 || s2_length == 0 {
        return 0;
    }

    if s1.eq(s2) {
        return 0;
    }
    let mut array: Vec<usize> = (1..).take(s1_length).collect();
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

//function to read file
fn read_file(file_name: String) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

//function to read file line by line and return a vector of strings
fn read_file_line_by_line(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents.lines().map(|s| s.to_string()).collect();
}

//function to merge two Stirng vectors
fn merge_vectors(mut v1: Vec<String>, mut v2: Vec<String>) -> Vec<String> {
    let mut v3 = Vec::new();
    v3.append(&mut v1);
    v3.append(&mut v2);
    return v3;
}



fn main() {
    let mut request:String = "GET /api/аrеu/v1/housenumber?muni=Chrysos&town=Chrysos&street=Quanderious%20Friederich&cyr=true&fields=house_number,town_name,muni_name,street_name".to_string();
    //let url_decoded_request: String = url_remove(request);
    // println!("Decoded URL: {}", url_decoded_request);

    let mut URL: String = "http://www.mysite.com/a%20file%20with%20spaces.html".to_string();

    let mut str: String = "test_string_123".to_string();
    let mut str2: String = "pest_spring_321".to_string();


    //tests
    //println!("\n\n\n");

    //println!("Test primer URL-a: {}\n", URL);
    //println!("Dekodovan URL: {}\n", url_decode(URL));
    //println!("URL sa uklonjenim URL-dekode karakterima: {}\n\n", url_remove(URL));
    
    //println!("Bigrami: {:?}", extract_unigrams(URL));
    //println!("Bigrami: {:?}", extract_bigrams(URL));
    //println!("Bigrami: {:?}", extract_trigrams(URL));
  
    /*println!("\n\n\n");
    println!("Test primer: {} i {}\n", str, str2);
    println!("Dice koeficijent slicnosti je: {:?}", dice_coefficient(&str, &str2));
    println!("Levenshtein koeficijent slicnosti je: {:?}", levenshtein_distance(&str, &str2));*/

    
    //println!("{:?}", read_file_line_by_line("test_file.txt".to_string()));

    //println!("{:?}", tokenize_string_by_special_character("test1_test2_test3_".to_string(), '_'));
    println!("{:?}", tokenize_string_by_string("______".to_string(), "__".to_string()));
}
