#[macro_use]
extern crate clap;
mod morse;
mod data;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use clap::App;
use data::Data;
use morse::Morse;

fn main() {

    let cli = load_yaml!(concat!(env!("CARGO_MANIFEST_DIR"), "/cli/default.yml"));
    let matches = App::from_yaml(cli).get_matches();

    if matches.is_present("input"){
        let path = matches.value_of("input").unwrap(); //Is present, so unwrap
        println!("Input: {:?}", path);
        let mut file = File::open(path).expect("Invalid input!");
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer).expect("Couldn't read the input file!");

        let msg = Data{bytes: buffer};
        println!("Hello, {}!", msg.to_morse());
    }else if matches.is_present("string"){
        let string = matches.value_of("string").unwrap(); //Is present, so unwrap
        println!("String: {:?}", string);
        let byts = String::from(string).into_bytes();

        let msg = Data{bytes: byts};
        println!("Hello, {}!", msg.to_morse());
    }

    if let Some(output) = matches.value_of("output"){
        println!("Output: {:?}", output);
    }
}
