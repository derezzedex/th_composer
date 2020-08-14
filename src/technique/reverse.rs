use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct ReverseEncodingError;

impl fmt::Display for ReverseEncodingError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reverse encoding error!")
    }
}

impl Error for ReverseEncodingError {}

pub trait Reversed<T>: Sized{
    fn encode(&mut self) -> Result<T, ReverseEncodingError>;

    fn decode(repr: T) -> Result<Self, ReverseEncodingError>;
}

use crate::data::Text;
impl Reversed<Text> for Text{
    fn encode(&mut self) -> Result<Text, ReverseEncodingError> {
        Ok(Text{
            content: self.content.chars().rev().collect()
        })
    }

    fn decode(repr: Text) -> Result<Self, ReverseEncodingError> {
        Ok(Text{
            content: repr.content.chars().rev().collect()
        })
    }
}
