#[derive(PartialEq, Debug)]
pub enum Token {
    Path(String),
    Param(String),
    Slash,
    QuestionMark,
    Ampersand,
    EqualSign,
    End,
}

#[derive(Clone)]
pub struct Lexer {
    chars: Vec<char>,
    cur: char,
    pos: usize,
    peek_pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let cur = chars[0];
        Self {
            chars,
            cur,
            pos: 0,
            peek_pos: 1,
        }
    }

    pub fn next_token(&mut self) -> Token {
        match self.cur {
            ':' => self.parse_param(),
            '/' => {
                self.read_char();
                Token::Slash
            }
            '\0' => {
                self.read_char();
                Token::End
            }
            '?' => {
                self.read_char();
                Token::QuestionMark
            }
            '&' => {
                self.read_char();
                Token::Ampersand
            }
            '=' => {
                self.read_char();
                Token::EqualSign
            }
            _ => self.parse_path(),
        }
    }

    fn read_char(&mut self) {
        let new_pos = (self.pos + 1).min(self.chars.len());
        let new_peek_pos = (new_pos + 1).min(self.chars.len());
        self.pos = new_pos;
        self.peek_pos = new_peek_pos;
        if new_pos >= self.chars.len() {
            self.cur = '\0';
        } else {
            self.cur = self.chars[self.pos];
        }
    }

    fn parse_path(&mut self) -> Token {
        let position = self.pos;
        while self.pos < self.chars.len() && valid_path_char(&self.cur) {
            self.read_char();
        }
        let path = String::from_iter(&self.chars[position..self.pos]);
        Token::Path(path)
    }

    fn parse_param(&mut self) -> Token {
        self.read_char();
        let position = self.pos;
        while self.pos < self.chars.len() && valid_path_char(&self.cur) {
            self.read_char();
        }
        let path = String::from_iter(&self.chars[position..self.pos]);
        Token::Param(path)
    }
}

fn valid_path_char(ch: &char) -> bool {
    let invalid_chars = [':', '/', '\0', '?', '=', '&'];
    !invalid_chars.contains(ch)
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    #[test]
    fn creates_tokens_from_path_string() {
        let path = "/:user/:id/preferences";
        let mut lexer = Lexer::new(path);

        assert_eq!(lexer.next_token(), Token::Slash);
        assert_eq!(lexer.next_token(), Token::Param("user".to_string()));
        assert_eq!(lexer.next_token(), Token::Slash);
        assert_eq!(lexer.next_token(), Token::Param("id".to_string()));
        assert_eq!(lexer.next_token(), Token::Slash);
        assert_eq!(lexer.next_token(), Token::Path("preferences".to_string()));
        assert_eq!(lexer.next_token(), Token::End);
    }

    #[test]
    fn default_index_path() {
        let path = "/";
        let mut lexer = Lexer::new(path);
        assert_eq!(lexer.next_token(), Token::Slash);

        let path = "";
        let mut lexer = Lexer::new(path);
        assert_eq!(lexer.next_token(), Token::Slash);
    }
}
