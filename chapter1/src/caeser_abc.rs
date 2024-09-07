fn encrypt(text: &str, shift: u16) -> String {
    let mut encrypted_text = String::new();
    let A_code = 'A' as u16;
    let Z_code = 'Z' as u16;
    let a_code = 'a' as u16;
    let z_code = 'z' as u16;

    for t in text.chars() {
        let t_code = t as u16;
        let crypted_code = if t_code >= A_code && t_code <= Z_code {
            (t_code - A_code + shift + 26) % 26 + A_code
        } else if t_code >= a_code && t_code <= z_code {
            (t_code - a_code + shift + 26) % 26 + a_code
        } else {
            t_code
        };
        let crypted_char = crypted_code as u8 as char;
        encrypted_text.push(crypted_char);
    }

    encrypted_text
}

fn main() {
    println!("{}", encrypt("HELLz", 3));
}
