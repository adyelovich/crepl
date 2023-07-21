use regex::Regex;
use std::collections::VecDeque;
use std::fmt;

struct CLangToken {
    pub regex: Regex,
    pub handle_match: fn(&str) -> Token,
}

impl CLangToken {
    fn new(regex: Regex, handle_match: fn(&str) -> Token) -> CLangToken {
        CLangToken {
            regex,
            handle_match,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Error<'a> {
    UnknownToken(&'a str)
}

impl <'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            UnknownToken(s) => write!(f, "Encountered unknown token: {}", s),
        }
    }
}


/*
Loop over all the regexes, for each one, check if it matches at
the start of the word, if it does then wrap it in its correpsonding
token, if not move onto the next one
*/
#[derive(PartialEq, Debug)]
pub enum Token {
    TokInt(i32),
    TokSemicolon,
}

impl Token {
    pub fn tokenize(line: &str) -> Result<VecDeque<Token>, Error<'_>> {
        let handlers = Self::init_handlers();
        let num_handlers = handlers.len();
        let mut toks: VecDeque<Token> = VecDeque::new();
        
        for word in line.split_ascii_whitespace() {
            let length = word.len();
            let mut pos = 0;
            let mut i = 0;
            loop {
                let handle = &handlers[i];
                match handle.regex.find_at(word, pos) {
                    Some(p) => if p.start() == pos {
                        //we found something and should capture it
                        toks.push_back((handle.handle_match)(&word[p.start()..p.end()]));
                        pos = p.end();
                        i = 0; //restart our checks
                    } else {
                        i += 1;
                    }
                    None => {i += 1},
                }

                if i == num_handlers {
                    // we reached the end of possible matches so stop here
                    break;
                }
            }

            if pos != length { //we did not reach the end, must be a lex error
                return Err(Error::UnknownToken(word));
            }
        }

        Ok(toks)
    }

    fn init_handlers() -> Vec<CLangToken> {
        vec![
            CLangToken::new(Regex::new(r"-?\d+").unwrap(),
                            |word| Token::TokInt(word.parse().unwrap())),
            CLangToken::new(Regex::new(";").unwrap(),
                            |_word| Token::TokSemicolon),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Token;
    use std::collections::VecDeque;
    
    #[test]
    fn tokenize_nothing() {
        assert_eq![Token::tokenize(""), Ok(VecDeque::from([]))];
    }
    
    #[test]
    fn tokenize_int32_single() {
        assert_eq![Token::tokenize("5"), Ok(VecDeque::from([Token::TokInt(5)]))];
    }

    #[test]
    fn tokenize_int32_multi() {
        assert_eq![Token::tokenize("36127"), Ok(VecDeque::from([Token::TokInt(36127)]))];
    }

    #[test]
    fn tokenize_int32_neg() {
        assert_eq![Token::tokenize("-42"), Ok(VecDeque::from([Token::TokInt(-42)]))];
    }

    #[test]
    fn tokenize_int32_semi() {
        assert_eq![Token::tokenize(";"), Ok(VecDeque::from([Token::TokSemicolon]))];
    }

    #[test]
    fn tokenize_int32_many() {
        use Token::*;
        assert_eq![Token::tokenize("42; 21;; 1;0;"),
                   Ok(VecDeque::from([TokInt(42), TokSemicolon, TokInt(21), TokSemicolon,
                        TokSemicolon,
                        TokInt(1), TokSemicolon, TokInt(0), TokSemicolon]))];
    }

    #[test]
    fn tokenize_wrong_int32() {
        let result = Token::tokenize("42n4");
        assert!(result.is_err());
    }
}
