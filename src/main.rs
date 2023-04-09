use num_cpus;
use pfind::document::Document;
use pfind::lexer::Lexer;
use std::collections::HashMap;
use std::env::args;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use walkdir::WalkDir;

// TODO: make this an optional commandline option
const PAGE_THRESH: usize = 25;

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

fn get_document(file_path: &PathBuf) -> Result<Document, ()> {
    let pdf = match poppler::PopplerDocument::new_from_file(file_path, "") {
        Err(err) => {
            eprintln!(
                "Could not open file '{file_path}': {err}",
                file_path = file_path.display()
            );
            return Err(());
        }
        Ok(pdf) => pdf,
    };

    let num_pages = pdf.get_n_pages();

    if num_pages > PAGE_THRESH {
        eprintln!(
            "Skipping {file_path}. Document too large: {num_pages} pages",
            file_path = file_path.display()
        );
        return Err(());
    }
    let mut doc = Document::new();
    for page_no in 0..num_pages {
        if let Some(page_text) = pdf
            .get_page(page_no)
            .expect("page number is valid")
            .get_text()
        {
            doc.push_page(Some(page_counts(&page_text)));
        }
    }
    Ok(doc)
}

fn worker_thread(thread_id: usize, tx: std::sync::mpsc::Sender<Document>) {}

fn main() {
    let in_path = PathBuf::from(args().nth(1).expect("No path given"));

    if !in_path.try_exists().unwrap_or(false) {
        eprintln!(
            "No such file or directory: '{path}'",
            path = in_path.display()
        );
    }
    println!("Reading {path}", path = in_path.display());

    if in_path.is_file() {
        let doc = get_document(&in_path);
    } else {
        let docs = Vec::<Document>::new();
        let files: Vec<PathBuf> = WalkDir::new(in_path)
            .into_iter()
            .filter_map(|v| v.ok())
            .filter_map(|v| {
                let path = PathBuf::from(v.path());
                if path.is_file() && path.extension().map_or(false, |ext| ext == "pdf") {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();
        let files = Arc::new(Mutex::new(files));

        let (tx, rx) = channel();

        let mut handles = Vec::new();
        for thread_id in 0..num_cpus::get() {
            let (files, tx) = (Arc::clone(&files), tx.clone());
            handles.push(thread::spawn(move || {
                println!("Hello from thread {thread_id}");
                loop {
                    let file = {
                        // TODO: handle errors
                        if let Ok(mut files) = files.try_lock() {
                            let file = files.pop();
                            drop(files);
                            file
                        } else {
                            eprintln!("{thread_id}: Could not acquire lock");
                            None
                        }
                    };
                    if let Some(file) = file {
                        match get_document(&file) {
                            Ok(doc) => tx.send(doc).unwrap(),
                            Err(_) => (),
                        }
                        println!("{thread_id} {file}", file = file.display());
                    } else {
                        // break if pop returns None. this means the queue is empty
                        break;
                    }
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
        for document in rx.try_iter() {
            // TODO: actually use results
        }
    }
}
