use std::env;
use std::error::Error;

// maximum entry possible: u64::MAX;
// 18446744073709551615

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next(); // bxc

    while let Some(arg) = args.next() {
        let arg = arg.to_lowercase();
        let num: u64;

        if arg.starts_with("0x") {
            num = bxc::hex_to_int(&arg[2..])?;    
        } else if arg.starts_with("0b") {
            num = bxc::bin_to_int(&arg[2..])?;
        } else {
            num = arg.parse()?;
        }

        println!("u64: {}\nhex: 0x{:X}\nbinary: {:b}\n", num, num, num);

    }

    Ok(())
}


