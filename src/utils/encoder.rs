pub trait Encodable<T>{
    type EncodeError;
    fn encode(&self, d: T) -> Result<T, Self::EncodeError>;
}
