use data::Data;
use encoder::Encodable;
use decoder::Decodable;

pub struct Morse;
const SEPARATOR: &str = " ";

#[derive(Debug)]
pub enum MorseError{
    InvalidChar(char),
}

impl Encodable<Data> for Morse{
    type EncodeError = MorseError;
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
                    .map(to_morse)
                    .map(|s| s.expect("Couldn't encode"))
                    .collect::<String>();

        Ok(Data::new(chars.into_bytes()))
    }
}

impl Decodable<Data> for Morse{
    type DecodeError = MorseError;
    fn decode(&self, data: Data) -> Result<Data, Self::DecodeError>{
        let chars = data
                    .to_string()
                    .split(SEPARATOR)
                    .into_iter()
                    .map(from_morse)
                    .map(|s| s.expect("Couldn't decode"))
                    .collect::<String>();

        Ok(Data::new(chars.into_bytes()))
    }
}

fn to_morse<'a>(chr: char) -> Result<&'a str, MorseError>{
    match chr {
        'A' => Ok(".−"),
        'B' => Ok("−..."),
        'C' => Ok("−.−."),
        'D' => Ok("−.."),
        'E' => Ok("."),
        'F' => Ok("..−."),
        'G' => Ok("−−."),
        'H' => Ok("...."),
        'I' => Ok(".."),
        'J' => Ok(".−−−"),
        'K' => Ok("−.−"),
        'L' => Ok(".−.."),
        'M' => Ok("−−"),
        'N' => Ok("−."),
        'O' => Ok("−−−"),
        'P' => Ok(".−−."),
        'Q' => Ok("−−.−"),
        'R' => Ok(".−."),
        'S' => Ok("..."),
        'T' => Ok("−"),
        'U' => Ok("..−"),
        'V' => Ok("...−"),
        'W' => Ok(".−−"),
        'X' => Ok("−..−"),
        'Y' => Ok("−.−−"),
        'Z' => Ok("−−.."),
        ' ' => Ok(" "),
        _ => Err(MorseError::InvalidChar(chr)),
    }
}

fn from_morse<'a>(morse: &'a str) -> Result<char, MorseError>{
    match morse {
         ".−" => Ok('A'),
         "−..." => Ok('B'),
         "−.−." => Ok('C'),
         "−.." => Ok('D'),
         "." => Ok('E'),
         "..−." => Ok('F'),
         "−−." => Ok('G'),
         "...." => Ok('H'),
         ".." => Ok('I'),
         ".−−−" => Ok('J'),
         "−.−" => Ok('K'),
         ".−.." => Ok('L'),
         "−−" => Ok('M'),
         "−." => Ok('N'),
         "−−−" => Ok('O'),
         ".−−." => Ok('P'),
         "−−.−" => Ok('Q'),
         ".−." => Ok('R'),
         "..." => Ok('S'),
         "−" => Ok('T'),
         "..−" => Ok('U'),
         "...−" => Ok('V'),
         ".−−" => Ok('W'),
         "−..−" => Ok('X'),
         "−.−−" => Ok('Y'),
         "−−.." => Ok('Z'),
         " " => Ok(' '),
         _ => Err(MorseError::InvalidChar(morse.chars().next().unwrap())),
    }
}
