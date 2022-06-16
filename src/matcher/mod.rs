mod lexer;

use std::collections::HashMap;

use lexer::Lexer;

use self::lexer::Token;

#[derive(Debug, Clone)]
pub struct RouteMatcher {
    path: String,
}

impl RouteMatcher {
    pub fn new(input: &str) -> Self {
        Self {
            path: input.to_owned(),
        }
    }

    pub fn matches(&self, input: &str) -> Option<HashMap<String, String>> {
        if self.straight_match(input) {
            Some(HashMap::<String, String>::new())
        } else {
            let mut route_lexer = Lexer::new(&self.path);
            let mut sym_lexer = Lexer::new(input);
            let mut map = HashMap::<String, String>::new();
            loop {
                let sym_t = sym_lexer.next_token();
                let route_t = route_lexer.next_token();
                match (route_t, sym_t) {
                    (Token::Param(k), Token::Path(v)) => {
                        map.insert(k, v);
                    }
                    (Token::Path(route), Token::Path(sym)) => {
                        if route != sym {
                            break None;
                        } else {
                            continue;
                        }
                    }
                    (Token::Slash, Token::Slash) => continue,
                    (Token::End, Token::End) => break Some(map),
                    _ => break None,
                }
            }
        }
    }

    fn straight_match(&self, input: &str) -> bool {
        input == &self.path
    }
}

#[cfg(test)]
mod test {
    use super::RouteMatcher;

    #[test]
    fn matches_without_params() {
        let matcher = RouteMatcher::new("/user/14/profile");

        if let Some(matches) = matcher.matches("/user/14/profile") {
            assert!(matches.is_empty());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn matches_params() {
        let matcher = RouteMatcher::new("/:user/:id/profile");

        if let Some(matches) = matcher.matches("/test_user/14/profile") {
            if let Some(user) = matches.get("user") {
                assert_eq!(user, "test_user");
            } else {
                assert!(false);
            };
            if let Some(user) = matches.get("id") {
                assert_eq!(user, "14");
            } else {
                assert!(false);
            };
        } else {
            assert!(false);
        }
    }
}
