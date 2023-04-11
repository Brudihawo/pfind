pub struct Lexer<'a> {
    content: &'a str,
    reached_end: bool,
}

impl<'a> Lexer<'a> {
    /// Construct a new Lexer
    ///
    /// * `content`: Text to tokenize
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            reached_end: false,
        }
    }

    /// skip ignored characters
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

    /// output next token. outputs None if end of token stream is reached
    fn next_token(&mut self) -> Option<String> {
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
            if tok.is_empty() {
                return None;
            }
        }
        self.content = content;
        return Some(tok.to_lowercase());
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;

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
            ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
        );
    }

    #[test]
    fn lexer_sentence_syms() {
        let lexer = Lexer::new("The / quick -+ +brown **fox jumps over the lazy dog.");
        let tokens = lexer.into_iter().collect::<Vec<_>>();
        assert_eq!(
            tokens,
            ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
        );
    }

    #[test]
    fn lexer_unicode() {
        let lexer = Lexer::new("The quick dogé jumps ovêrre le lazy fròge.");
        let tokens = lexer.into_iter().collect::<Vec<_>>();
        assert_eq!(
            tokens,
            ["the", "quick", "dogé", "jumps", "ovêrre", "le", "lazy", "fròge"]
        );
    }

    #[test]
    fn empty() {
        let lexer = Lexer::new("");
        let tokens = lexer.into_iter().collect::<Vec<_>>();
        assert_eq!(tokens, Vec::<&str>::new());
    }

    #[test]
    fn one() {
        let lexer = Lexer::new("The");
        let tokens = lexer.into_iter().collect::<Vec<_>>();
        assert_eq!(tokens, ["the"]);
    }
}
