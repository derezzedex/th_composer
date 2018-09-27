use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn read_file(path: &str) -> Vec<u8>{
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut buffer: Vec<u8> = Vec::new();
    match file.read_to_end(&mut buffer){
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(_) => buffer,
    }
}

pub fn write_file(path: &str, buffer: &[u8]){
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::create(&path){
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(buffer){
        Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
        Ok(_) => {},
    }
}
