use std::collections::HashMap;

pub struct Document {
    page_occurrences: Vec<Option<HashMap<String, usize>>>,
    // TODO: make page_occurrences and document_occurrences reference a word list to save memory
    document_occurrences: HashMap<String, usize>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            document_occurrences: HashMap::new(),
            page_occurrences: Vec::new(),
        }
    }

    pub fn push_page(&mut self, page_occurrences: Option<HashMap<String, usize>>) {
        if let Some(page_occurrences) = page_occurrences {
            for (token, occ) in page_occurrences.iter() {
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
}
