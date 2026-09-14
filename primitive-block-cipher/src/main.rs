use std::io::{self, Write};

const BLOCK_SIZE: usize = 4;

const SBOX: [u8; 16] = [
    0xC, 0x5, 0x6, 0xB,
    0x9, 0x0, 0xA, 0xD,
    0x3, 0xE, 0xF, 0x8,
    0x4, 0x7, 0x1, 0x2,
];

const INV_SBOX: [u8; 16] = [
    0x5, 0xE, 0xF, 0x8,
    0xC, 0x1, 0x2, 0xD,
    0xB, 0x4, 0x6, 0x3,
    0x0, 0x7, 0x9, 0xA,
];

fn input(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();

    let mut value = String::new();
    io::stdin().read_line(&mut value).unwrap();

    value.trim_end().to_string()
}

fn substitute(byte: u8) -> u8 {
    let high = SBOX[(byte >> 4) as usize];
    let low = SBOX[(byte & 0x0F) as usize];

    (high << 4) | low
}

fn inverse_substitute(byte: u8) -> u8 {
    let high = INV_SBOX[(byte >> 4) as usize];
    let low = INV_SBOX[(byte & 0x0F) as usize];

    (high << 4) | low
}

fn encrypt(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();

    for (block_number, block) in plaintext.chunks(BLOCK_SIZE).enumerate() {
        let mut block = block.to_vec();

        for byte in &mut block {
            *byte = substitute(*byte);  // substitution
        }

        for i in 0..block.len() {
            let key_index = block_number * BLOCK_SIZE + i;
            block[i] ^= key[key_index % key.len()];  // XOR with key
        }

        block.reverse();  // transposition

        result.extend(block);
    }

    result
}

fn decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();

    for (block_number, block) in ciphertext.chunks(BLOCK_SIZE).enumerate() {
        let mut block = block.to_vec();

        block.reverse();  // transpose

        for i in 0..block.len() {
            let key_index = block_number * BLOCK_SIZE + i;
            block[i] ^= key[key_index % key.len()];  // XOR with key
        }

        for byte in &mut block {
            *byte = inverse_substitute(*byte);  // inverse substitution
        }

        result.extend(block);
    }

    result
}

fn to_hex(data: &[u8]) -> String {  // convert input to hex
    data.iter()
        .map(|byte| format!("{:02X}", byte))
        .collect()
}

fn from_hex(hex: &str) -> Vec<u8> {
    let mut result = Vec::new();

    for i in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..i + 2], 16).unwrap();
        result.push(byte);
    }

    result
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