use std::fmt;
use std::error::Error;

use crate::technique::tools;

#[derive(Debug, Clone)]
pub struct CaesarEncodingError;

impl fmt::Display for CaesarEncodingError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Caesar encoding error!")
    }
}

impl Error for CaesarEncodingError {}

pub trait Caesar<T>: Sized{
    fn encode(&mut self) -> Result<T, CaesarEncodingError>;

    fn decode(repr: T) -> Result<Self, CaesarEncodingError>;
}

use crate::data::Text;
impl Caesar<Text> for Text{
    fn encode(&mut self) -> Result<Text, CaesarEncodingError> {
        Ok(Text{
            content: self.content.chars().map(|c| tools::shift(c, 3)).collect()
        })
    }

    fn decode(repr: Text) -> Result<Self, CaesarEncodingError> {
        Ok(Text{
            content: repr.content.chars().map(|c| tools::shift(c, -3)).collect()
        })
    }
}
