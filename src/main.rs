/*
 * Copyright (c) 2024, İbrahim UYSAL <uysalibov@gmail.com>
 *
 * MIT License
 */

use papyrust::parser::parser::{print_raw_pdf, read_pdf};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: papyrust <filename>");
        return;
    }
    let filename = &args[1];

    let pdf_content = match read_pdf(filename) {
        Ok(content) => content,
        Err(e) => panic!("{}", e),
    };

    print_raw_pdf(&pdf_content);
}
