use super::token::*;

fn is_register(word: &&str) -> bool {
    // TODO
    word.starts_with("r")
}

pub fn tokenize(line: &str, tokens: &mut Vec<Token>) {
    let line = line.replace(",", " ");
    let words: Vec<&str> = line.split_whitespace().collect();
    let is_ignore_line = |t: &Vec<&str>| {
        t.len() == 0 || (t.len() > 0 && t[0] == "#")
    };

    if is_ignore_line(&words) {
        return;
    }

    for word in words {
        if let Ok(num) = word.parse::<i32>() {
            let t = Token::new(TokenKind::INT(num));
            tokens.push(t);
        } else if is_register(&word) {
            let t = Token::new(TokenKind::REG(word.clone().to_string()));
            tokens.push(t);
        } else {
            let t = match word {
                "ADD"  => Token::new(TokenKind::ADD ),
                "ADDI" => Token::new(TokenKind::ADDI),
                "SUB"  => Token::new(TokenKind::SUB ),
                "XOR"  => Token::new(TokenKind::XOR ),
                _ => panic!("Invalid token: \"{}\"", word),
            };
            tokens.push(t);
        }
    }

    tokens.push(Token::new(TokenKind::EOL));
}
