use m68kdecode;
use m68kdecode::{DecodedInstruction};

use std::fs;
use std::fs::File;
use std::io::Read;
use std::env;

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn usage() {
    eprintln!("Usage: m68kdecodec <filename> 0x<hex_address>");
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        usage();
        return;
    }
    let file_name = &args[1];

    // There has to be a better way to convert str(0x1234) => usize(4660)
    let addr_str = &args[2].trim();
    let address = usize::from_str_radix(&addr_str[2..], 16).expect("Address must be numeric in hex format: 0xADDRESS");


    let buffer = get_file_as_byte_vec(&file_name);
    let r = m68kdecode::decode_instruction(&buffer[address..]);

    match r {
        Err(e) => {
            println!("Got: {:?}", e);
            assert!(false);
        }
        Ok(DecodedInstruction {
            bytes_used,
            instruction,
        }) => {
            println!("{:02X?} => {:?}", &buffer[address..(address + bytes_used as usize)], instruction);
        }
    }
}