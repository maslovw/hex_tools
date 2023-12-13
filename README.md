# Hex Tools

set off CLI tools to work with bits and bytes

## get_byte

This tool will take input string of hex values and print specified in the command line byte number of an input

Example:

    $echo "01020304" | get_byte 1
    > 0x02

This will print value of bit 1 in byte 1:

    $ echo "01020304" | get_byte 1.1
    > 1

### USAGE

    get_byte.exe [OPTIONS] <byte-number>

#### OPTIONS

    -f, --format <format>
            Format of output: bin, dec, hex [default: hex]

    -h, --hex-string <hex-string>
            Hex string

## set_byte

This tool will take input string of hex values and print specified in the command line byte number of an input

### USAGE

    set_byte.exe [OPTIONS] <byte-number> [value]

#### OPTIONS

    -h, --hex-string <hex-string>    Hex string

#### ARGS

    <byte-number>    Byte number
    <value>          Value to set [default: 1]

## print_bits

Tool with print positions of the set bits in the provided value respectively to offset
(bit counting from offset)

### USAGE

    print_bits <hex_value> [offset]

#### ARGS

    <hex_value>      Value for evaluation
    [offset]         offset for bit counting [default: 1]

### EXAMPLE

    > print_bits 0x103 1
    *Bit  1 is set
    *Bit  2 is set
    *Bit  9 is set


# Build

    cargo build
