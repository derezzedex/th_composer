extern crate clap;
#[macro_use]
extern crate data_encoding_macro;
extern crate data_encoding;

mod utils;
mod techniques;

use clap::App;

use utils::data::Data;
use utils::encoder::Encodable;
use utils::decoder::Decodable;
use utils::fs;

use techniques::morse::Morse;
use techniques::reverse::Reverse;
use techniques::keypad::Keypad;
use techniques::base::{Base2, Base8, UpperBase16, LowerBase16, Base32, Base64};

const DEFAULT_YAML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/cli/default.yml"));
enum Techniques{
    Morse(Morse),
    Reverse(Reverse),
    Keypad(Keypad),
    Binary(Base2),
    Octal(Base8),
    LowerHex(LowerBase16),
    UpperHex(UpperBase16),
    Base32(Base32),
    Base64(Base64),
}

fn main() {
    let cli = clap::YamlLoader::load_from_str(DEFAULT_YAML).expect("Couldn't include YAML file");
    let matches = App::from_yaml(&cli[0]).get_matches();

    let mut data: Option<Data> = None;
    if matches.is_present("input"){
        let path = matches.value_of("input").unwrap();
        let buffer = fs::read_file(path);

        data = Some(Data::new(buffer));
    }else if matches.is_present("string"){
        let string = matches.value_of("string").unwrap();
        let bytes = String::from(string).into_bytes();

        data = Some(Data::new(bytes));
    }

    if let Some(d) = &data{
        let technique = match matches.value_of("technique"){
            Some("morse") => Techniques::Morse(Morse),
            Some("reverse") => Techniques::Reverse(Reverse),
            Some("keypad") => Techniques::Keypad(Keypad),
            Some("binary") => Techniques::Binary(Base2),
            Some("octal") => Techniques::Octal(Base8),
            Some("lowerhex") => Techniques::LowerHex(LowerBase16),
            Some("upperhex") => Techniques::UpperHex(UpperBase16),
            Some("base32") => Techniques::Base32(Base32),
            Some("base64") => Techniques::Base64(Base64),
            _ => unreachable!(),
        };

        let mut buffer: Option<Data> = None;
        if matches.is_present("encode"){
            let b = match technique{
                Techniques::Morse(m) => m.encode(d.clone()).expect("Couldn't encode"),
                Techniques::Reverse(r) => r.encode(d.clone()).expect("Couldn't encode"),
                Techniques::Keypad(k) => k.encode(d.clone()).expect("Couldn't encode"),
                Techniques::Binary(b) => b.encode(d.clone()).expect("Couldn't encode"),
                Techniques::Octal(o) => o.encode(d.clone()).expect("Couldn't encode"),
                Techniques::LowerHex(lh) => lh.encode(d.clone()).expect("Couldn't encode"),
                Techniques::UpperHex(uh) => uh.encode(d.clone()).expect("Couldn't encode"),
                Techniques::Base32(b32) => b32.encode(d.clone()).expect("Couldn't encode"),
                Techniques::Base64(b64) => b64.encode(d.clone()).expect("Couldn't encode"),
            };
            buffer = Some(b);

        }else if matches.is_present("decode"){
            let b = match technique{
                Techniques::Morse(m) => m.decode(d.clone()).expect("Couldn't decode"),
                Techniques::Reverse(r) => r.decode(d.clone()).expect("Couldn't decode"),
                Techniques::Keypad(k) => k.decode(d.clone()).expect("Couldn't decode"),
                Techniques::Binary(b) => b.decode(d.clone()).expect("Couldn't encode"),
                Techniques::Octal(o) => o.decode(d.clone()).expect("Couldn't encode"),
                Techniques::LowerHex(lh) => lh.decode(d.clone()).expect("Couldn't encode"),
                Techniques::UpperHex(uh) => uh.decode(d.clone()).expect("Couldn't encode"),
                Techniques::Base32(b32) => b32.decode(d.clone()).expect("Couldn't encode"),
                Techniques::Base64(b64) => b64.decode(d.clone()).expect("Couldn't encode"),
            };
            buffer = Some(b);

        }
        if let Some(buffer) = buffer{
            if let Some(output) = matches.value_of("output"){
                fs::write_file(output, buffer.bytes());
            }else{
                println!("{}", buffer);
            }
        }else{
            panic!("Only Encode/Decode options are valid!");
        }
    }
}
