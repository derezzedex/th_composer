#[macro_use]
extern crate clap;
mod encoder;
mod decoder;
mod data;

mod morse;

use std::io::prelude::*;
use std::fs::File;
use clap::App;
use data::Data;

use morse::Morse;

fn main() {

    let cli = load_yaml!(concat!(env!("CARGO_MANIFEST_DIR"), "/cli/default.yml"));
    let matches = App::from_yaml(cli).get_matches();

    let mut data: Option<Data> = None;
    if matches.is_present("input"){
        let path = matches.value_of("input").unwrap(); //Is present, so unwrap
        println!("Input: {:?}", path);
        let mut file = File::open(path).expect("Invalid input!");
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer).expect("Couldn't read the input file!");

        data = Some(Data::new(buffer));
    }else if matches.is_present("string"){
        let string = matches.value_of("string").unwrap(); //Is present, so unwrap
        println!("String: {:?}", string);
        let bytes = String::from(string).into_bytes();

        data = Some(Data::new(bytes));
    }

    if let Some(data) = data{
        if matches.is_present("encode"){
            println!("Encoded: {}", encoder::encode(Morse, data).expect("Couldn't encode"));
        }else if matches.is_present("decode"){
            println!("Decoded: {}", decoder::decode(Morse, data).expect("Couldn't decode"));
        }
    }

    if let Some(output) = matches.value_of("output"){
        println!("Output: {:?}", output);
    }
}
