extern crate clap;
mod encoder;
mod decoder;
mod data;

mod morse;
mod reverse;

use std::io::prelude::*;
use std::fs::File;
use clap::App;
use data::Data;

use morse::*;
use reverse::*;

use encoder::Encodable;
use decoder::Decodable;

const DEFAULT_YAML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/cli/default.yml"));

fn main() {

    let cli = clap::YamlLoader::load_from_str(DEFAULT_YAML).expect("Couldn't include YAML file");
    let matches = App::from_yaml(&cli[0]).get_matches();

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
            let technique: Box<Encodable<Data>> = match matches.value_of("technique"){
                Some("morse") => Box::new(Morse),
                Some("reverse") => Box::new(Reverse),
                e => panic!("Not implemented: {:?}", e.unwrap()),
            };
            println!("Encoded: {}", encoder::encode(&*technique, data).expect("Couldn't encode"));
        }else if matches.is_present("decode"){
            let technique: Box<Decodable<Data>> = match matches.value_of("technique"){
                Some("morse") => Box::new(Morse),
                Some("reverse") => Box::new(Reverse),
                _ => Box::new(Morse),
            };
            println!("Decoded: {}", decoder::decode(&*technique, data).expect("Couldn't decode"));
        }
    }

    if let Some(output) = matches.value_of("output"){
        println!("Output: {:?}", output);
    }
}
