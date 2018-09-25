use data::Data;
use encoder::Encodable;
use decoder::Decodable;

pub struct Reverse;
pub enum ReverseError{}

impl Encodable<Data> for Reverse{
    type EncodeError = ReverseError;
    fn encode(&self, data: Data) -> Result<Data, Self::EncodeError>{
        let chars = data
                    .to_string()
                    .chars()
                    .into_iter()
                    .rev()
                    .collect::<String>();

        Ok(Data::new(chars.into_bytes()))
    }
}

impl Decodable<Data> for Reverse{
    type DecodeError = ReverseError;
    fn decode(&self, data: Data) -> Result<Data, Self::DecodeError>{
        let chars = data
                    .to_string()
                    .chars()
                    .into_iter()
                    .rev()
                    .collect::<String>();

        Ok(Data::new(chars.into_bytes()))
    }
}
