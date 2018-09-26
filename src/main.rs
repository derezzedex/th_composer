extern crate clap;

mod utils;
mod techniques;

use std::io::prelude::*;
use std::fs::File;
use clap::App;

use utils::data::Data;
use utils::encoder::Encodable;
use utils::decoder::Decodable;

use techniques::morse::Morse;
use techniques::reverse::Reverse;
use techniques::keypad::Keypad;
use techniques::base::{Base2, Base8, Base32, Base64};

const DEFAULT_YAML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/cli/default.yml"));
enum Techniques{
    Morse(Morse),
    Reverse(Reverse),
    Keypad(Keypad),
    Binary(Base2),
    Octal(Base8),
    Base32(Base32),
    Base64(Base64),
}

fn main() {

    let cli = clap::YamlLoader::load_from_str(DEFAULT_YAML).expect("Couldn't include YAML file");
    let matches = App::from_yaml(&cli[0]).get_matches();

    let mut data: Option<Data> = None;
    if matches.is_present("input"){
        let path = matches.value_of("input").unwrap(); //Is present, so unwrap
        //println!("Input: {:?}", path);
        let mut file = File::open(path).expect("Invalid input!");
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer).expect("Couldn't read the input file!");

        data = Some(Data::new(buffer));
    }else if matches.is_present("string"){
        let string = matches.value_of("string").unwrap(); //Is present, so unwrap
        //println!("String: {:?}", string);
        let bytes = String::from(string).into_bytes();

        data = Some(Data::new(bytes));
    }

    if let Some(data) = data{
        let technique = match matches.value_of("technique"){
            Some("morse") => Techniques::Morse(Morse),
            Some("reverse") => Techniques::Reverse(Reverse),
            Some("keypad") => Techniques::Keypad(Keypad),
            _ => unreachable!(),
        };

        if matches.is_present("encode"){
            let encoded = match technique{
                Techniques::Morse(m) => m.encode(data).expect("Couldn't encode"),
                Techniques::Reverse(r) => r.encode(data).expect("Couldn't encode"),
                Techniques::Keypad(k) => k.encode(data).expect("Couldn't encode"),
            };

            println!("{}", encoded); //TODO: Add Debug mode
        }else if matches.is_present("decode"){
            let decoded = match technique{
                Techniques::Morse(m) => m.decode(data).expect("Couldn't decode"),
                Techniques::Reverse(r) => r.decode(data).expect("Couldn't decode"),
                Techniques::Keypad(k) => k.decode(data).expect("Couldn't decode"),
            };
            println!("{}", decoded); //TODO: Add Debug mode
        }
    }

    if let Some(output) = matches.value_of("output"){
        println!("Output: {:?}", output);
    }
}
