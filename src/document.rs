use std::collections::HashMap;

/// Struct representing the occurrences of words in a document.
/// The document is built up while iterating the pages of the document using a pdf parsing library
///
/// * `page_occurrences`: occurrences of words per-page
/// * `document_occurrences`: occurrences of words in a document
pub struct Document {
    page_occurrences: Vec<Option<HashMap<String, usize>>>,
    // TODO: make page_occurrences and document_occurrences reference a word list to save memory
    document_occurrences: HashMap<String, usize>,
    word_count: usize,
}

impl Document {
    pub fn new() -> Self {
        Self {
            document_occurrences: HashMap::new(),
            page_occurrences: Vec::new(),
            word_count: 0,
        }
    }

    /// Add term frequency mapping for a page in the document
    ///
    /// * `page_occurrences`: term frequency mapping for the page. None if page has no words
    pub fn push_page(&mut self, page_occurrences: Option<HashMap<String, usize>>) {
        if let Some(page_occurrences) = page_occurrences {
            for (token, occ) in page_occurrences.iter() {
                self.word_count += occ;
                if let Some(num) = self.document_occurrences.get_mut(token) {
                    *num += occ;
                } else {
                    self.document_occurrences.insert(token.to_string(), *occ);
                }
            }
            self.page_occurrences.push(Some(page_occurrences));
        } else {
            self.page_occurrences.push(None);
        }
    }

    /// get term occurrences in document
    ///
    /// * `term`: term to compute frequency for
    pub fn occurrences(&self, term: &str) -> usize {
        self.document_occurrences.get(term).map(|x| *x).unwrap_or(0)
    }
}

struct DocumentSet {
    docs: Vec<Document>,
}

impl DocumentSet {
    pub fn tf_idf(&self, term: &str) {
        let occurrences: Vec<usize> = self.docs.iter().map(|doc| doc.occurrences(term)).collect();
        let all_occs = 
    }
}
