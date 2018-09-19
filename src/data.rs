use std::fmt;

pub struct Data{
    pub bytes: Vec<u8>,
}

impl fmt::Display for Data{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", String::from_utf8_lossy(&self.bytes))
    }
}
