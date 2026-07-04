use std::error::Error;

pub fn hex_to_int(num_str: &str) -> Result<u64, Box<dyn Error>> {
    let mut bytes: Vec<u8> = String::from(num_str).into_bytes();
    bytes.reverse();

    let mut num: u64 = 0;
    for (i, n) in bytes.iter().enumerate() {
        let n = match n {
            b'0'..=b'9' => n - b'0',
            b'a'..=b'f' => n - b'a' + 10,
            b'A'..=b'F' => n - b'A' + 10,
            _ => return Err("Must supply valid hex input".into()),
        };
        num += (n as u64) * 16_u64.pow(i as u32);
    }
    Ok(num)
}



pub fn bin_to_int(num_str: &str) -> Result<u64, Box<dyn Error>> {
    let mut bytes: Vec<u8> = String::from(num_str).into_bytes();
    bytes.reverse();

    let mut num: u64 = 0;
    for (i, n) in bytes.iter().enumerate() {
        match n {
            b'0'..=b'1' => num += (n - b'0') as u64 * 2_u64.pow(i as u32),
            _ => return Err("must supply valid binary input".into()),
        }
    }

    Ok(num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_to_int_correct() {
        let cases = vec![
            ("0001", 1),
            ("0010", 2),
            ("0100", 4),
            ("1000", 8),
        ];

        for (input, expected) in cases {
            let output = bin_to_int(input).expect("should not panic");
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn bin_to_int_incorrect() {
        let cases = vec![
            "a",
            "00002",
            "lol",
            "!",
        ];
        for input in cases {
            let output = bin_to_int(input);
            assert_eq!(output.unwrap_err().to_string(), "must supply valid binary input");
        }
    }

}
