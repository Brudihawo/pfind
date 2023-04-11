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
    file: String,
}

impl Document {
    pub fn new(fname: String) -> Self {
        Self {
            document_occurrences: HashMap::new(),
            page_occurrences: Vec::new(),
            word_count: 0,
            file: fname,
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

pub struct DocumentSet {
    docs: Vec<Document>,
}

impl DocumentSet {
    pub fn new(docs: Vec<Document>) -> Self {
        Self { docs }
    }

    pub fn get_name(&self, idx: usize) -> &str {
        &self.docs[idx].file
    }

    pub fn tf_idf(&self, terms: Vec<String>) -> Vec<f64> {
        let num_docs = self.docs.len();
        let mut occs_tf = Vec::<(usize, f64)>::with_capacity(num_docs);
        let mut tf_idf: Vec<f64> = (0..num_docs).map(|_| 0.0).collect();

        for term in terms.iter() {
            occs_tf.extend(self.docs.iter().map(|doc| {
                let occs = doc.occurrences(term);
                (
                    occs,
                    if doc.word_count != 0 {
                        occs as f64 / doc.word_count as f64
                    } else {
                        0.0
                    },
                )
            }));

            let occs_total = occs_tf
                .iter()
                .fold(0, |acc, (occ, _)| if occ > &0 { acc + 1 } else { acc });
            let idf = (num_docs as f64 / (1.0 + occs_total as f64)).log10();
            let idf = if idf == std::f64::NAN { 0.0 } else { idf };
            for (val, (_, tf)) in tf_idf.iter_mut().zip(&occs_tf) {
                *val += tf * idf;
            }
            occs_tf.clear();
        }
        tf_idf
    }
}
