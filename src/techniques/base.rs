use utils::data::Data;
use utils::encoder::Encodable;
use utils::decoder::Decodable;

use std::char;

pub struct Base2;
pub struct Base8;
pub struct Base32;
pub struct Base64;
const SEPARATOR: &str = " ";

/*
fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}
*/

#[derive(Debug)]
pub enum BaseError{
    InvalidChar(char),
}

impl Encodable<Data> for Base2{
    type EncodeError = BaseError;
    fn encode(&self, data: Data) -> Result<Data, Self::EncodeError>{
        let chars = data
                    .to_string()
                    .chars()
                    .collect::<String>();

        Ok(Data::new(chars.into_bytes()))
    }
}

impl Decodable<Data> for Base2{
    type DecodeError = BaseError;
    fn decode(&self, data: Data) -> Result<Data, Self::DecodeError>{
        let chars = data
                    .to_string()
                    .chars()
                    .collect::<String>();

        Ok(Data::new(chars.into_bytes()))
    }
}
