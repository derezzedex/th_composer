pub trait Decodable<T>{
    type DecodeError;
    fn decode(&self, d: T) -> Result<T, Self::DecodeError>;
}
