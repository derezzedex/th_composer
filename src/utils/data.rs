use std::fmt;

#[derive(Clone, Debug)]
pub struct Data{
    bytes: Vec<u8>,
}

impl fmt::Display for Data{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", String::from_utf8_lossy(&self.bytes))
    }
}

impl Data{
    pub fn new(b: Vec<u8>) -> Data{
        Data{
            bytes: b,
        }
    }

    pub fn mut_bytes(&mut self) -> &mut [u8]{
        &mut self.bytes[..]
    }

    pub fn bytes(&self) -> &[u8]{
        &self.bytes[..]
    }
}
