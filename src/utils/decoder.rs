use utils::data::Data;

pub trait Decodable<T>{
    type DecodeError;
    fn decode(&self, d: T) -> Result<T, Self::DecodeError>;
}

pub fn decode<T: ?Sized + Decodable<Data>>(t: &T, d: Data) -> Result<Data, T::DecodeError>{
    t.decode(d)
}