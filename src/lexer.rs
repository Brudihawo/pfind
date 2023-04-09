pub struct Lexer<'a> {
    content: &'a str,
    reached_end: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            reached_end: false,
        }
    }

    fn skip_chars(&mut self) {
        let mut index = 0;
        for chr in self.content.chars() {
            if !chr.is_alphanumeric() {
                index += chr.len_utf8();
                continue;
            }
            break;
        }
        let (_, content) = self.content.split_at(index);
        self.content = content;
    }

    fn next_token(&mut self) -> Option<&'a str> {
        if self.reached_end {
            return None;
        }

        self.skip_chars();
        let mut index = 0;
        for chr in self.content.chars() {
            if chr.is_alphanumeric() {
                index += chr.len_utf8();
                continue;
            }
            break;
        }

        let (tok, content) = self.content.split_at(index);
        if content.is_empty() {
            self.reached_end = true;
            return None;
        }
        self.content = content;
        return Some(tok);
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lexer_sentence() {
        let lexer = Lexer::new("The quick brown fox jumps over the lazy dog.");
        let tokens = lexer.into_iter().collect::<Vec<_>>();
        assert_eq!(
            tokens,
            ["The", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
        );
    }

    #[test]
    fn lexer_sentence_syms() {
        let lexer = Lexer::new("The / quick -+ +brown **fox jumps over the lazy dog.");
        let tokens = lexer.into_iter().collect::<Vec<_>>();
        assert_eq!(
            tokens,
            ["The", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
        );
    }

    #[test]
    fn lexer_unicode() {
        let lexer = Lexer::new("The quick dogé jumps ovêrre le lazy fròge.");
        let tokens = lexer.into_iter().collect::<Vec<_>>();
        assert_eq!(
            tokens,
            ["The", "quick", "dogé", "jumps", "ovêrre", "le", "lazy", "fròge"]
        );
    }
}
