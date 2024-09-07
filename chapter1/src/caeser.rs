fn encrypt(text: &str, shift: u32) -> String {
    let mut encrypted_text = String::new();
    for t in text.chars() {
        let t_encoded = std::char::from_u32(t as u32 + shift).unwrap();
        encrypted_text.push(t_encoded);
    }
    encrypted_text
}

fn decrypt(text: &str, shift: u32) -> String {
    let mut decrypted_text = String::new();
    for t in text.chars() {
        let t_decoded = std::char::from_u32(t as u32 - shift).unwrap();
        decrypted_text.push(t_decoded);
    }
    decrypted_text
}

fn main() {
    let encrypt_sample = encrypt("おはなすいた", 1);
    println!("{}", encrypt_sample);
    let decrypt_sample = decrypt(&encrypt_sample, 1);
    println!("{}", decrypt_sample);
    println!("{}", "おはなすいた" == decrypt_sample);
}
