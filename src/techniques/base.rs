use utils::data::Data;
use utils::encoder::Encodable;
use utils::decoder::Decodable;

use data_encoding::{Encoding};
use data_encoding::HEXLOWER;
use data_encoding::HEXUPPER;
use data_encoding::BASE32;
use data_encoding::BASE64;

pub struct Base2;
const BASE2: Encoding = new_encoding!{
    symbols: "01",
    wrap_width: 8,
    wrap_separator: " ",
};

pub struct Base8;
const SEPARATOR: &str = " ";
const BASE8: Encoding = new_encoding!{
    symbols: "01234567",
};

pub struct LowerBase16;
pub struct UpperBase16;
pub struct Base32;
pub struct Base64;

#[derive(Debug)]
pub enum BaseError{}

impl Encodable<Data> for Base2{
    type EncodeError = BaseError;
    fn encode(&self, data: Data) -> Result<Data, Self::EncodeError>{
        let mut datac = data.clone();
        let bytes = datac.mut_bytes();
        let mut buffer: Vec<u8> = vec![0; BASE2.encode_len(bytes.len())];
        BASE2.encode_mut(bytes, &mut buffer[..]);
        Ok(Data::new(buffer))
    }
}

impl Decodable<Data> for Base2{
    type DecodeError = BaseError;
    fn decode(&self, data: Data) -> Result<Data, Self::DecodeError>{
        let mut datac = data.clone();
        let bytes = datac.mut_bytes();
        let mut buffer: Vec<u8> = vec![0; BASE2.decode_len(bytes.len()).unwrap()];
        let len = BASE2.decode_mut(bytes, &mut buffer[..]).expect("Couldn't decode");
        buffer.truncate(len);
        Ok(Data::new(buffer))
    }
}

/*
impl Encodable<Data> for Base8{
    type EncodeError = BaseError;
    fn encode(&self, data: Data) -> Result<Data, Self::EncodeError>{
        let mut datac = data.clone();
        let bytes = datac.mut_bytes();
        let mut buffer: Vec<u8> = vec![0; BASE8.encode_len(bytes.len())];
        BASE8.encode_mut(bytes, &mut buffer[..]);
        Ok(Data::new(buffer))
    }
}
*/



impl Encodable<Data> for Base8{
    type EncodeError = BaseError;
    fn encode(&self, data: Data) -> Result<Data, Self::EncodeError>{
        let datac = data.clone();
        let mut b = [0; 4];
        let buffer = datac
                    .to_string()
                    .chars()
                    .into_iter()
                    .map(|c| {
                        let len = c.encode_utf8(&mut b).len();
                        let mut result = HEXUPPER.encode(&b[0..len]);
                        result.push_str(SEPARATOR);
                        println!("{:?}: {:o}", c, &result.parse::<u8>().unwrap());
                        result
                    })
                    .collect::<String>();

        Ok(Data::new(buffer.into_bytes()))
    }
}


impl Decodable<Data> for Base8{
    type DecodeError = BaseError;
    fn decode(&self, data: Data) -> Result<Data, Self::DecodeError>{
        let mut datac = data.clone();
        let bytes = datac.mut_bytes();
        let mut buffer: Vec<u8> = vec![0; BASE8.decode_len(bytes.len()).unwrap()];
        let len = BASE8.decode_mut(bytes, &mut buffer[..]).expect("Couldn't decode");
        buffer.truncate(len);
        Ok(Data::new(buffer))
    }
}

impl Encodable<Data> for LowerBase16{
    type EncodeError = BaseError;
    fn encode(&self, data: Data) -> Result<Data, Self::EncodeError>{
        let mut datac = data.clone();
        let bytes = datac.mut_bytes();
        let mut buffer: Vec<u8> = vec![0; HEXLOWER.encode_len(bytes.len())];
        HEXLOWER.encode_mut(bytes, &mut buffer[..]);
        Ok(Data::new(buffer))
    }
}

impl Decodable<Data> for LowerBase16{
    type DecodeError = BaseError;
    fn decode(&self, data: Data) -> Result<Data, Self::DecodeError>{
        let mut datac = data.clone();
        let bytes = datac.mut_bytes();
        let mut buffer: Vec<u8> = vec![0; HEXLOWER.decode_len(bytes.len()).unwrap()];
        let len = HEXLOWER.decode_mut(bytes, &mut buffer[..]).expect("Couldn't decode");
        buffer.truncate(len);
        Ok(Data::new(buffer))
    }
}

impl Encodable<Data> for UpperBase16{
    type EncodeError = BaseError;
    fn encode(&self, data: Data) -> Result<Data, Self::EncodeError>{
        let mut datac = data.clone();
        let bytes = datac.mut_bytes();
        let mut buffer: Vec<u8> = vec![0; HEXUPPER.encode_len(bytes.len())];
        HEXUPPER.encode_mut(bytes, &mut buffer[..]);
        Ok(Data::new(buffer))
    }
}

impl Decodable<Data> for UpperBase16{
    type DecodeError = BaseError;
    fn decode(&self, data: Data) -> Result<Data, Self::DecodeError>{
        let mut datac = data.clone();
        let bytes = datac.mut_bytes();
        let mut buffer: Vec<u8> = vec![0; HEXUPPER.decode_len(bytes.len()).unwrap()];
        let len = HEXUPPER.decode_mut(bytes, &mut buffer[..]).expect("Couldn't decode");
        buffer.truncate(len);
        Ok(Data::new(buffer))
    }
}

impl Encodable<Data> for Base32{
    type EncodeError = BaseError;
    fn encode(&self, data: Data) -> Result<Data, Self::EncodeError>{
        let mut datac = data.clone();
        let bytes = datac.mut_bytes();
        let mut buffer: Vec<u8> = vec![0; BASE32.encode_len(bytes.len())];
        BASE32.encode_mut(bytes, &mut buffer[..]);
        Ok(Data::new(buffer))
    }
}

impl Decodable<Data> for Base32{
    type DecodeError = BaseError;
    fn decode(&self, data: Data) -> Result<Data, Self::DecodeError>{
        let mut datac = data.clone();
        let bytes = datac.mut_bytes();
        let mut buffer: Vec<u8> = vec![0; BASE32.decode_len(bytes.len()).unwrap()];
        let len = BASE32.decode_mut(bytes, &mut buffer[..]).expect("Couldn't decode");
        buffer.truncate(len);
        Ok(Data::new(buffer))
    }
}

impl Encodable<Data> for Base64{
    type EncodeError = BaseError;
    fn encode(&self, data: Data) -> Result<Data, Self::EncodeError>{
        let mut datac = data.clone();
        let bytes = datac.mut_bytes();
        let mut buffer: Vec<u8> = vec![0; BASE64.encode_len(bytes.len())];
        BASE64.encode_mut(bytes, &mut buffer[..]);
        Ok(Data::new(buffer))
    }
}

impl Decodable<Data> for Base64{
    type DecodeError = BaseError;
    fn decode(&self, data: Data) -> Result<Data, Self::DecodeError>{
        let mut datac = data.clone();
        let bytes = datac.mut_bytes();
        let mut buffer: Vec<u8> = vec![0; BASE64.decode_len(bytes.len()).unwrap()];
        let len = BASE64.decode_mut(bytes, &mut buffer[..]).expect("Couldn't decode");
        buffer.truncate(len);
        Ok(Data::new(buffer))
    }
}
