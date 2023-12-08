use std::env; 

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: print_bits <hex_value> [offset: default=1]");
        std::process::exit(1)
    }

    let hex_value = match u64::from_str_radix(&args[1].trim_start_matches("0x"), 16) {
        Ok(val)=>val,
        Err(_) => { 
            eprintln!("Invalid value");
            std::process::exit(1);
        }
    };
    let offset = if args.len() == 3 { 
        match args[2].parse::<u64>() {
            Ok(val) => val, 
            Err(_) => {
                eprintln!("Invalid value");
                std::process::exit(1);
            }
        }
    } else {
        1
    };

    print_set_bits(hex_value, offset);
}

fn print_set_bits(value: u64, offset: u64) {
    for i in 0..64 {
        if (value & (1 << i)) != 0 {
            println!("*Bit {:>2} is set", i + offset);
        }
    }
}
