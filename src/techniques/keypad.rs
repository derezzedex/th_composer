use utils::data::Data;
use utils::encoder::Encodable;
use utils::decoder::Decodable;

pub struct Keypad;
const SEPARATOR: &str = " ";

#[derive(Debug)]
pub enum KeypadError{
    InvalidChar(char),
}

impl Encodable<Data> for Keypad{
    type EncodeError = KeypadError;
    fn encode(&self, data: Data) -> Result<Data, Self::EncodeError>{
        let chars = data
                    .to_string()
                    .to_uppercase()
                    .chars()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(SEPARATOR)
                    .chars()
                    .into_iter()
                    .map(to_keypad)
                    .map(|s| s.expect("Couldn't encode"))
                    .collect::<String>();

        Ok(Data::new(chars.into_bytes()))
    }
}

impl Decodable<Data> for Keypad{
    type DecodeError = KeypadError;
    fn decode(&self, data: Data) -> Result<Data, Self::DecodeError>{
        let chars = data
                    .to_string()
                    .split(SEPARATOR)
                    .into_iter()
                    .map(from_keypad)
                    .map(|s| s.expect("Couldn't decode"))
                    .collect::<String>();

        Ok(Data::new(chars.into_bytes()))
    }
}

fn to_keypad<'a>(chr: char) -> Result<&'a str, KeypadError>{
    match chr {
        'A' => Ok("2"),
        'B' => Ok("22"),
        'C' => Ok("222"),
        'D' => Ok("3"),
        'E' => Ok("33"),
        'F' => Ok("333"),
        'G' => Ok("4"),
        'H' => Ok("44"),
        'I' => Ok("444"),
        'J' => Ok("5"),
        'K' => Ok("55"),
        'L' => Ok("555"),
        'M' => Ok("6"),
        'N' => Ok("66"),
        'O' => Ok("666"),
        'P' => Ok("7"),
        'Q' => Ok("77"),
        'R' => Ok("777"),
        'S' => Ok("7777"),
        'T' => Ok("8"),
        'U' => Ok("88"),
        'V' => Ok("888"),
        'W' => Ok("9"),
        'X' => Ok("99"),
        'Y' => Ok("999"),
        'Z' => Ok("9999"),
        ' ' => Ok(" "),
        _ => Err(KeypadError::InvalidChar(chr)),
    }
}

fn from_keypad<'a>(keypad: &'a str) -> Result<char, KeypadError>{
    match keypad {
         "2" => Ok('A'),
         "22" => Ok('B'),
         "222" => Ok('C'),
         "3" => Ok('D'),
         "33" => Ok('E'),
         "333" => Ok('F'),
         "4" => Ok('G'),
         "44" => Ok('H'),
         "444" => Ok('I'),
         "5" => Ok('J'),
         "55" => Ok('K'),
         "555" => Ok('L'),
         "6" => Ok('M'),
         "66" => Ok('N'),
         "666" => Ok('O'),
         "7" => Ok('P'),
         "77" => Ok('Q'),
         "777" => Ok('R'),
         "7777" => Ok('S'),
         "8" => Ok('T'),
         "88" => Ok('U'),
         "888" => Ok('V'),
         "9" => Ok('W'),
         "99" => Ok('X'),
         "999" => Ok('Y'),
         "9999" => Ok('Z'),
         " "    => Ok(' '),
         _ => Err(KeypadError::InvalidChar(keypad.chars().next().unwrap())),
    }
}
