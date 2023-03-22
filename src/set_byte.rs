use std::io::{self, Read};
use std::error::Error;
use std::str::FromStr;
use structopt::StructOpt;

fn hex_to_bin(hex_string: &str) -> Vec<u8> {
    hex::decode(hex_string).unwrap()
}

fn bin_to_hex(bin_string: &[u8]) -> String {
    hex::encode(bin_string)
}

fn get_byte(bin_string: &[u8], byte_number: usize) -> u8 {
    bin_string[byte_number]
}

#[derive(Debug, StructOpt)]
#[structopt(name = "set_byte")]
struct Opt {
    #[structopt(help = "Byte number")]
    byte_number: String,

    #[structopt(default_value = "1", help = "Value to set")]
    value: String,

    #[structopt(short, long, help = "Hex string")]
    hex_string: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();


    let vhex = opt.hex_string.unwrap_or_else(|| {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        buffer.trim().to_owned()
    });

    
    let (byte, bit) = if opt.byte_number.contains('.') {
        let byte_bit: Vec<&str> = opt.byte_number.split('.').collect();
        let byte = byte_bit[0].parse().unwrap();
        let bit: u8 = byte_bit[1].parse().unwrap();
        (byte, Some(bit))
    } else {
        let byte = opt.byte_number.parse().unwrap();
        (byte, None)
    };

    let vbin = hex_to_bin(&vhex);
    let vbyte = get_byte(&vbin, byte);

    // transform opt.value to u8, default in decimal, if prefix 0x is used, it will be in hex
    let opt_value;
    if opt.value.starts_with("0x") {
        opt_value = u8::from_str_radix(&opt.value[2..], 16).unwrap();
    } else {
        opt_value = u8::from_str(&opt.value).unwrap();
    }

    let value = if let Some(bit) = bit {
        if opt_value == 1 {
            vbyte | (1 << bit)
        } else if opt_value == 0 {
            // cast bit to u8
            let b = bit as u8;

            vbyte & !(1 << b)
        } else {
            panic!("Value for bit must be 0 or 1");
        }
    } else {
        opt_value
    };

    let mut vbin_mut = vbin.clone();
    vbin_mut[byte] = value;
    let vhex_out = bin_to_hex(&vbin_mut);

    println!("{}", vhex_out);

    Ok(())
}

