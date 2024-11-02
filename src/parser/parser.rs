/*
 * Copyright (c) 2024, İbrahim UYSAL <uysalibov@gmail.com>
 *
 * MIT License
 */

use std::{
    fs::File,
    io::{self, Read},
};

pub fn read_pdf(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    if !std::path::Path::new(filename).exists() {
        eprintln!("Error: File not found: {}", filename);
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found!"));
    }

    let mut file = File::open(filename)?;

    let mut contents = Vec::new();
    match file.read_to_end(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => {
            eprintln!("Error: While reading PDF file: {:?}", e);
            Err(e)
        }
    }
}

pub fn print_raw_pdf(content: &Vec<u8>) {
    for byte in content {
        if let Some(c) = std::char::from_u32(*byte as u32) {
            print!("{}", c);
        } else {
            print!("?");
        }
    }
}
