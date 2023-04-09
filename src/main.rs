use pfind::document::Document;
use pfind::lexer::Lexer;
use std::collections::HashMap;
use std::env::args;

fn page_counts(text: &str) -> HashMap<String, usize> {
    let mut ret = HashMap::new();
    for token in Lexer::new(text) {
        if let Some(occurrences) = ret.get_mut(token) {
            *occurrences += 1;
        } else {
            ret.insert(token.to_string(), 1);
        }
    }
    ret
}

fn main() {
    let file_path = args().nth(1).expect("No name given");

    println!("Reading {file_path}");

    let pdf = match poppler::PopplerDocument::new_from_file(&file_path, "") {
        Err(err) => {
            eprintln!("Could not open file '{file_path}': {err}");
            return;
        }
        Ok(pdf) => pdf,
    };

    let mut doc = Document::new();
    for page_no in 0..pdf.get_n_pages() {
        if let Some(page_text) = pdf
            .get_page(page_no)
            .expect("page number is valid")
            .get_text()
        {
            doc.push_page(Some(page_counts(&page_text)));
        }
    }
}
