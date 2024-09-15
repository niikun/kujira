fn encrypt(text: &str, shift: i16) -> String {
    let a = 'A' as i16;
    let is_az = |c| 'A' <= c && c <= 'Z';
    let conv = |c| (((c - a + shift + 26) % 26 + a) as u8) as char;
    let enc1 = |c| if is_az(c) { conv(c as i16) } else { c };
    return text.chars().map(|c| enc1(c)).collect();
}

fn main() {
    let encrypt_sample = encrypt("HELLO", 3);
    println!("{}", encrypt_sample);
    let decrypt_sample = encrypt(&encrypt_sample, -3);
    println!("{}", decrypt_sample);
    println!("{}", "HELLO" == decrypt_sample);
}
