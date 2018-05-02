use std::slice::Iter;

fn main() {
    loop {
        // Read input from the user and just do nothing when the input is empty
        let input = read_string();
        if input.is_empty() {
            continue;
        } else {
            let mut tokens = match Token::tokenize(&input) {
                Ok(ret) => ret,
                Err(e) => {
                    println!("{}", e.description());
                    continue;
                }
            };
            let expr = match Expr::parse(&mut tokens) {
                Ok(ret) => ret,
                Err(e) => {
                    println!("{}", e.description());
                    continue;
                }
            };
            println!("{} = {}", input, expr.evaluate());
        }
    }
}

/// Exceptions, which can occure during tokenization
#[derive(Clone, Copy, Debug)]
enum LexError {
    Overflow,
    UnexpectedChar(char),
}

impl LexError {
    fn description(&self) -> String {
        match *self {
            LexError::UnexpectedChar(c) => format!("Unexpected character: {}", c),
            LexError::Overflow => String::from("Integer overflow"),
        }
    }
}

/// Exceptions, which can occur during parsing
#[derive(Clone, Copy, Debug)]
enum ParseError {
    UnexpectedToken(Token),
    MissingParen,
    UnexpectedEol,
    UnnecessaryToken,
}

impl ParseError {
    fn description(&self) -> String {
        match *self {
            ParseError::UnexpectedToken(ref t) => format!("Invalid token {:?} found", t),
            ParseError::MissingParen => String::from("Closing ')' missing"),
            ParseError::UnexpectedEol => String::from("Unexpected end of input string"),
            ParseError::UnnecessaryToken => String::from("Unmatched Tokens"),
        }
    }
}

/// Representation of a token
#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    Plus,
    Minus,
    OpenParen,
    CloseParen,
    Space,
    Number(i32),
}

impl Token {
    /// Parse the input string and generate tokens
    fn tokenize(line: &str) -> Result<Vec<Self>, LexError> {
        /// helper function for numbers (multiple characters)
        fn push_number(vec: &mut Vec<Token>, num: String) -> Result<String, LexError> {
            if !num.is_empty() {
                match num.parse() {
                    Ok(ret) => vec.push(Token::Number(ret)),
                    _ => return Err(LexError::Overflow),
                }
            }
            Ok(String::new())
        }

        let mut ret = Vec::new();
        let mut input = String::new();

        for c in line.chars() {
            let token = match c {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                c if c.is_whitespace() => Token::Space,
                c @ '0'...'9' => {
                    input.push(c);
                    continue;
                }
                _ => return Err(LexError::UnexpectedChar(c)),
            };

            input = match push_number(&mut ret, input) {
                Ok(ret) => ret,
                Err(e) => return Err(e),
            };
            if Token::Space != token {
                ret.push(token);
            }
        }

        match push_number(&mut ret, input) {
            Err(e) => Err(e),
            _ => Ok(ret),
        }
    }
}

/// Representation of an operator
#[derive(Clone, Copy, Debug)]
enum Op {
    Plus,
    Minus,
}

impl Op {
    /// apply operator to two numbers
    fn apply(&self, left: i32, right: i32) -> i32 {
        match *self {
            Op::Plus => left + right,
            Op::Minus => left - right,
        }
    }
}

#[derive(Clone, Debug)]
enum Operand {
    Num(i32),
    Exp(Box<Expr>),
}

impl Operand {
    fn get(&self) -> i32 {
        match self {
            &Operand::Num(num) => num,
            &Operand::Exp(ref expr) => (*expr).evaluate(),
        }
    }
}

#[derive(Clone, Debug)]
enum Expr {
    Single(Operand),
    Compound {
        op: Op,
        left: Operand,
        right: Operand,
    },
}

impl Expr {
    fn evaluate(&self) -> i32 {
        match *self {
            Expr::Single(ref v) => v.get(),
            Expr::Compound {
                ref op,
                ref left,
                ref right,
            } => op.apply(left.get(), right.get()),
        }
    }

    /// convert the token string to an parsetree
    fn parse(input: &[Token]) -> Result<Expr, ParseError> {
        /// parse a possible expression
        ///```
        ///expr    := ⟨operand⟩ [⟨op⟩ ⟨operand⟩]
        ///```
        fn parse_expr<'a>(iter: &mut Iter<'a, Token>) -> Result<Expr, ParseError> {
            let lhs = match parse_operand(iter) {
                Ok(ret) => ret,
                Err(e) => return Err(e),
            };

            // early return if only left hand side exists
            match iter.as_slice().first() {
                Some(&Token::CloseParen) | None => return Ok(Expr::Single(lhs)),
                _ => {}
            }

            let op = match iter.next() {
                Some(&Token::Plus) => Op::Plus,
                Some(&Token::Minus) => Op::Minus,
                Some(t) => return Err(ParseError::UnexpectedToken(*t)),
                None => return Err(ParseError::UnexpectedEol),
            };
            let rhs = match parse_operand(iter) {
                Ok(ret) => ret,
                Err(e) => return Err(e),
            };

            Ok(Expr::Compound {
                op: op,
                left: lhs,
                right: rhs,
            })
        }

        /// parse one operand
        /// ```
        /// operand := ⟨num⟩ | "(" ⟨expr⟩ ")"
        /// ```
        fn parse_operand<'a>(iter: &mut Iter<'a, Token>) -> Result<Operand, ParseError> {
            let token = match iter.next() {
                None => return Err(ParseError::UnexpectedEol),
                Some(ret) => *ret,
            };

            match token {
                Token::Number(n) => return Ok(Operand::Num(n)),
                Token::OpenParen => match parse_expr(iter) {
                    Err(e) => return Err(e),
                    Ok(ret) => match iter.next() {
                        Some(&Token::CloseParen) => Ok(Operand::Exp(Box::new(ret))),
                        _ => Err(ParseError::MissingParen),
                    },
                },
                _ => return Err(ParseError::UnexpectedToken(token)),
            }
        }

        // create iterator and start recursive parsing
        let iter = &mut input.iter();
        let ret = parse_expr(iter);

        match (ret, iter.next()) {
            (Ok(ret), None) => Ok(ret), // result ok and nothing left: check
            (Ok(_), _) => Err(ParseError::UnnecessaryToken), // result ok, but token left
            (Err(e), _) => Err(e),      // some error during expression parsing
        }
    }
}

/// Reads a string from the user (with a nice prompt).
fn read_string() -> String {
    use std::io::Write;

    // Print prompt
    print!("calc > ");
    std::io::stdout().flush().unwrap();

    // Read line
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_right().len();
    buffer.truncate(new_len);

    buffer
}

#[test]
fn test_expr() {
    let r = Expr::Compound {
        op: Op::Plus,
        left: Operand::Num(4),
        right: Operand::Num(5),
    };
    let l = Expr::Compound {
        op: Op::Plus,
        left: Operand::Num(6),
        right: Operand::Num(5),
    };
    let t2 = Expr::Compound {
        op: Op::Minus,
        left: Operand::Exp(Box::new(l)),
        right: Operand::Exp(Box::new(r)),
    };
    assert_eq!(t2.evaluate(), 2);
}
