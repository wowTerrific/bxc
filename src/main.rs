use std::env;
use std::error::Error;

fn hex_to_int(num_str: &str) -> Result<u64, Box<dyn Error>> {
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next(); // bxc
    

    while let Some(arg) = args.next() {
        let arg = arg.to_lowercase();
        let num: u64;

        // TODO: not taking into account binary input
        if arg.starts_with("0x") {
            num = hex_to_int(&arg[2..])?;    
        } else {
            num = arg.parse()?;
        }

        println!("u64: {}\nhex: 0x{:X}\nbinary: {:b}\n", num, num, num);

    }

    Ok(())
    
}


