use std::env;

const USAGE: &str = "
Usage: bxc INT1 INT2 INT3...

bxc can transform a list of binary, hexadecimal, and positive integers. 

Values are prefixed the same way they are in the Rust language. Hexidecimal values are case insensitive. bxc has a max value of 184467440473709551615 due to the constraints of the u64 type. 
    Hexidecimal:    '0x<value>' - ex. 0x883f3A
    Binary:         '0b<value>' - ex. 0b1100 0b100101
    Integer:        '<value>'   - ex. 999 123 0154

Example usage:
    `bxc 0xf32a 0b0011 254 0xEEEE 0b10000000 9999999`
";

fn main() {
    let mut args = env::args();

    args.next(); // bxc

    while let Some(arg) = args.next() {
        let arg = arg.to_lowercase();
        let num: u64;

        if arg.starts_with("0x") {
            num = bxc::hex_to_int(&arg[2..]).expect(USAGE);    
        } else if arg.starts_with("0b") {
            num = bxc::bin_to_int(&arg[2..]).expect(USAGE);
        } else {
            num = arg.parse().expect(USAGE);
        }

        println!("u64: {}\nhex: 0x{:X}\nbinary: {:b}\n", num, num, num);
    }

}


