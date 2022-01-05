fn get_char(string: String, i: usize) -> char {
    return string.chars().nth(i).unwrap();
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

fn main() {
    let request:String = "GET /api/аrеu/v1/housenumber?muni=Chrysos&town=Chrysos&street=Quanderious%20Friederich&cyr=true&fields=house_number,town_name,muni_name,street_name".to_string();
    let url_decoded_request: String = url_remove(request);
    println!("Decoded URL: {}", url_decoded_request);
}
