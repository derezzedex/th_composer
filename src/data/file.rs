use std::fs::File as StandardFile;
use std::io::prelude::*;
use std::path::Path;

use anyhow::Result;

#[derive(Debug)]
pub struct File{
    pub content: StandardFile,
}

impl File{
    pub fn new<P: AsRef<Path>>(path: P) -> Result<File>{
        Ok(File{
            content: StandardFile::create(path)?,
        })
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<File>{
        Ok(File{
            content: StandardFile::open(path)?,
        })
    }

    pub fn read(&mut self) -> Result<Vec<u8>>{
        let mut content = Vec::new();
        self.content.read_to_end(&mut content)?;
        Ok(content)
    }

    pub fn write(&mut self, data: &[u8]) -> Result<()>{
        self.content.write_all(data)?;
        Ok(())
    }
}

// impl From<String> for Text{
//     fn from(item: String) -> Self {
//         Text{ content: item }
//     }
// }
