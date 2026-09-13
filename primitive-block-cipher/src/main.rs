use std::io::{self, Write};

const BLOCK_SIZE: usize = 4;

fn input(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();

    let mut value = String::new();
    io::stdin().read_line(&mut value).unwrap();

    value.trim_end().to_string()
}

fn encrypt(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();

    for (block_number, block) in plaintext.chunks(BLOCK_SIZE).enumerate() {
        let mut block = block.to_vec();

        // transposition
        block.reverse();

        // XOR with key
        for i in 0..block.len() {
            let key_index = block_number * BLOCK_SIZE + i;
            block[i] ^= key[key_index % key.len()];
        }

        result.extend(block);
    }

    result
}

fn decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();

    for (block_number, block) in ciphertext.chunks(BLOCK_SIZE).enumerate() {
        let mut block = block.to_vec();

        // XOR with key
        for i in 0..block.len() {
            let key_index = block_number * BLOCK_SIZE + i;
            block[i] ^= key[key_index % key.len()];
        }

        // transposition
        block.reverse();

        result.extend(block);
    }

    result
}

fn to_hex(data: &[u8]) -> String {
    data.iter()
        .map(|byte| format!("{:02X}", byte))
        .collect()
}

fn from_hex(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

fn main() {
    println!("1 - Verschlüsseln");
    println!("2 - Entschlüsseln");

    let choice = input("Auswahl: ");

    match choice.as_str() {
        "1" => {
            let plaintext = input("Klartext: ");
            let key = input("Schlüssel: ");

            let ciphertext =
                encrypt(plaintext.as_bytes(), key.as_bytes());

            println!();
            println!("Ciphertext:");
            println!("{}", to_hex(&ciphertext));
        }

        "2" => {
            let ciphertext = input("Ciphertext: ");
            let key = input("Schlüssel: ");

            let ciphertext = from_hex(&ciphertext);

            let plaintext =
                decrypt(&ciphertext, key.as_bytes());

            println!();
            println!("Klartext:");
            println!("{}", String::from_utf8_lossy(&plaintext));
        }

        _ => {
            println!("Ungültige Auswahl.");
        }
    }
}