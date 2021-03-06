fn get_char(string: String, i: usize) -> char {
    return string.chars().nth(i).unwrap();
}

fn substr(str: String, pos: usize, len: i32) -> String {
    let ss: String = str.chars().skip(pos).take(len as usize).collect();
    return ss;
}

fn process_decoded_string(src: String, operation: i32) -> String {
    // operation = 0 -> decode, operation = 1 -> remove
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

fn main() {
    let mut request:String = "GET /api/??r??u/v1/housenumber?muni=Chrysos&town=Chrysos&street=Quanderious%20Friederich&cyr=true&fields=house_number,town_name,muni_name,street_name".to_string();
    //let url_decoded_request: String = url_remove(request);
    // println!("Decoded URL: {}", url_decoded_request);
    let mut str: String = "test".to_string();
    println!("{:?}", dice_coefficient("testing", "pesping"));
}
