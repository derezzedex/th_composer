use data::Data;

pub trait Encodable<T>{
    type EncodeError;
    fn encode(&self, d: T) -> Result<T, Self::EncodeError>;
}

pub fn encode<T: Encodable<Data>>(t: T, d: Data) -> Result<Data, T::EncodeError>{
    t.encode(d)
}
