/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code: String = code.chars().filter(|c| !c.is_whitespace()).rev().collect();

    if code.chars().any(|c| !c.is_ascii_digit()) || code.len() <= 1 {
        return false;
    }

    let mut count: u8 = 0;
    let sum: u32 = code
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| {
            count += 1;

            if count % 2 == 0 {
                let mut d = d * 2;
                if d > 9 {
                    d = d - 9
                }

                d
            } else {
                d
            }
        })
        .sum();

    sum % 10 == 0
}
