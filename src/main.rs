use mupdf::document::Document;
use pfind::lexer::Lexer;
use std::env::args;
use std::error::Error;

fn tokenize_page(text: &str) -> Vec<&str> {
    Lexer::new(text).into_iter().collect()
}

fn main() {
    let val = "tèsté";

    for (i, chr) in val.char_indices() {
        // let cb = val.is_char_boundary(i);
        // let isalnum = chr.is_ascii_alphanumeric();
        println!("{i}, {chr}");//, {cb} -> {isalnum}")
    }
}

fn main2() -> Result<(), Box<dyn Error>> {
    let filename = args().nth(1).expect("No name given");

    println!("Reading {filename}");

    let document = Document::open(&filename)?;
    if !document.is_pdf() {
        eprintln!("{filename} is not a PDF. Exiting.");
        return Ok(());
    }

    if document.needs_password().unwrap() {
        eprintln!("{filename} needs password.")
    }

    for page in document.pages()? {
        let page_text = page?.to_text()?;
        let tokens = tokenize_page(&page_text);
        print!("{tokens:?}")
    }
    Ok(())
}
