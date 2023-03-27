use std::io::{self, Read};
use std::str::FromStr;
use structopt::StructOpt;

/// This script will take input string of hex values and print specified in
/// the command line byte number of an input
///
/// Example:
///
///     $echo "01020304" | get_byte 1
///
///     >0x02
///
/// This will print value of bit 1 in byte 1:
///
///     $echo "01020304" | get_byte 1.1
///
///     >1
#[derive(Debug, StructOpt)]
#[structopt(name = "get_byte")]
struct Opt {
    /// Hex string
    #[structopt(short, long)]
    hex_string: Option<String>,

    /// Byte number to print (or byte number and bit number separated by a period)
    byte_number: String,

    /// Format of output: bin, dec, hex
    #[structopt(short, long, default_value = "hex")]
    format: String,
}

fn hex_to_bin(hex_string: &str) -> Vec<u8> {
    hex::decode(hex_string).unwrap()
}

fn get_byte(bin_string: &[u8], byte_number: usize) -> u8 {
    bin_string[byte_number]
}

fn main() {
    let opt = Opt::from_args();

    let vhex = opt.hex_string.unwrap_or_else(|| {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        buffer.trim().to_owned()
    });

    let byte_number_str = opt.byte_number;
    let byte_number = if let Some(dot_index) = byte_number_str.find('.') {
        let byte_str = &byte_number_str[..dot_index];
        let bit_str = &byte_number_str[dot_index + 1..];
        let byte = usize::from_str(byte_str).unwrap();
        let bit = usize::from_str(bit_str).unwrap();
        let mask = 1 << bit;
        (byte, Some(mask))
    } else {
        (usize::from_str(&byte_number_str).unwrap(), None)
    };

    let vbin = hex_to_bin(&vhex);
    let byte = get_byte(&vbin, byte_number.0);

    // type of mask is Option<u8>
    if let Some(mask) = byte_number.1 {
        // convert maks to u32
        let m = mask as u32;
        let bit_value = (byte >> m.trailing_zeros()) & 1;
        println!("{}", bit_value);
    } else {
        match opt.format.as_str() {
            "bin" => println!("{:08b}", byte),
            "dec" => println!("{}", byte),
            "hex" => println!("0x{:02X}", byte),
            _ => panic!("Invalid format: {}", opt.format),
        }
    }
}
