fn map(c: &u8) -> u8 {
    if c.is_ascii_digit() {
        return *c;
    }

    219 - *c
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    decode(plain).chars()
        .collect::<Vec<_>>()
        .as_slice()
        .chunks(5)
        .collect::<Vec<_>>()
        .as_slice()
        .join(&' ')
        .iter()
        .collect::<String>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    String::from_utf8(cipher.
        replace(|x: char| !x.is_alphanumeric() || !x.is_ascii(), "").
        to_lowercase().
        as_bytes().
        iter().
        map(map).
        collect::<Vec<u8>>()
    ).unwrap()
}
