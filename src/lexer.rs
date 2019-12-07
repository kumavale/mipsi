use super::token::*;

fn is_register(word: &&str) -> Result<(RegisterKind, usize), String> {
    if word.as_bytes()[0] != b'$' {
        return Err(format!("Invalid register name: {}", word));
    }

    let mut identifier = word.to_string();
    identifier.remove(0);  // Delete '$'

    let (register_kind, idx) = match &*identifier {
        "zero" |  "0" => (RegisterKind::zero, 0),
        "at"   |  "1" => (RegisterKind::at,   1),
        "v0"   |  "2" => (RegisterKind::v0,   2),
        "v1"   |  "3" => (RegisterKind::v1,   3),
        "a0"   |  "4" => (RegisterKind::a0,   4),
        "a1"   |  "5" => (RegisterKind::a1,   5),
        "a2"   |  "6" => (RegisterKind::a2,   6),
        "a3"   |  "7" => (RegisterKind::a3,   7),
        "t0"   |  "8" => (RegisterKind::t0,   8),
        "t1"   |  "9" => (RegisterKind::t1,   9),
        "t2"   | "10" => (RegisterKind::t2,  10),
        "t3"   | "11" => (RegisterKind::t3,  11),
        "t4"   | "12" => (RegisterKind::t4,  12),
        "t5"   | "13" => (RegisterKind::t5,  13),
        "t6"   | "14" => (RegisterKind::t6,  14),
        "t7"   | "15" => (RegisterKind::t7,  15),
        "s0"   | "16" => (RegisterKind::s0,  16),
        "s1"   | "17" => (RegisterKind::s1,  17),
        "s2"   | "18" => (RegisterKind::s2,  18),
        "s3"   | "19" => (RegisterKind::s3,  19),
        "s4"   | "20" => (RegisterKind::s4,  20),
        "s5"   | "21" => (RegisterKind::s5,  21),
        "s6"   | "22" => (RegisterKind::s6,  22),
        "s7"   | "23" => (RegisterKind::s7,  23),
        "t8"   | "24" => (RegisterKind::t8,  24),
        "t9"   | "25" => (RegisterKind::t9,  25),
        "k0"   | "26" => (RegisterKind::k0,  26),
        "k1"   | "27" => (RegisterKind::k1,  27),
        "gp"   | "28" => (RegisterKind::gp,  28),
        "sp"   | "29" => (RegisterKind::sp,  29),
        "fp"   | "30" => (RegisterKind::fp,  30),
        "ra"   | "31" => (RegisterKind::ra,  31),
        _ => return Err(format!("Invalid register name: {}", word)),
    };

    Ok((register_kind, idx))
}

fn is_label(word: &&str) -> bool {
    word.ends_with(":")
}

pub fn tokenize(number_of_lines: u32, line: &str, tokens: &mut Tokens) {
    let line = line.replace(",", " ");
    let words: Vec<&str> = line.split_whitespace().collect();

    // Skip blank line either comment line
    if words.len() == 0 || words.len() > 0 && words[0] == "#" {
        return;
    }

    for word in words {
        if let Ok(num) = word.parse::<i32>() {
            tokens.push(TokenKind::INTEGER(num), number_of_lines);
        } else if let Ok((k, i)) = is_register(&word) {
            tokens.push(TokenKind::REGISTER(k, i), number_of_lines);
        } else {
            match &*word.to_ascii_uppercase() {
                // Arithmetic, Logic
                "ADD"  => tokens.push(TokenKind::INSTRUCTION(InstructionKind::ADD),  number_of_lines),
                "ADDI" => tokens.push(TokenKind::INSTRUCTION(InstructionKind::ADDI), number_of_lines),
                "SUB"  => tokens.push(TokenKind::INSTRUCTION(InstructionKind::SUB),  number_of_lines),
                "XOR"  => tokens.push(TokenKind::INSTRUCTION(InstructionKind::XOR),  number_of_lines),
                // Constant
                "LI"   => tokens.push(TokenKind::INSTRUCTION(InstructionKind::LI),   number_of_lines),
                // Comparison
                // Branch, Jump
                "BLT"  => tokens.push(TokenKind::INSTRUCTION(InstructionKind::BLT),  number_of_lines),
                // Load, Store
                // Transfer
                "MOVE" => tokens.push(TokenKind::INSTRUCTION(InstructionKind::MOVE), number_of_lines),
                // Exception, Interrupt
                "SYSCALL" => tokens.push(TokenKind::INSTRUCTION(InstructionKind::SYSCALL), number_of_lines),
                "#"    => break,
                _ => {
                    if is_label(&word) {
                        let mut identifier = word.to_string();
                        identifier.remove(identifier.len()-1);  // Delete ':'
                        tokens.push(TokenKind::LABEL(identifier, tokens.len()), number_of_lines);
                    } else {
                        tokens.push(TokenKind::ADDRESS(word.to_string()), number_of_lines);
                    }
                },
            }
        }
    }

    tokens.push(TokenKind::EOL, number_of_lines);
}

#[test]
#[cfg(test)]
fn test_tokenize() {
    use std::io::{BufRead, BufReader};

    let input = "\
# This is comment.
main:
    ADDI    $0,     $31,    256
    add	$t1,	$t2,	$t3
    SUB     $t4,    $t5,    $t6
    Xor     $t1,    $t1,    $t1
    LI      $v0,    1
    MOVE    $a0,    $t2
    syscall
    syscall  # Here is comment too
    BLT     $t0,    $t1,    label
";

    let mut tokens: Tokens = Tokens::new();
    let mut buf = String::new();
    let mut reader = BufReader::new(input.as_bytes());
    while reader.read_line(&mut buf).unwrap() > 0 {
        tokenize(0, &buf, &mut tokens);
        buf.clear();
    }

    assert_eq!(tokens.consume().unwrap().0, TokenKind::LABEL("main".to_string(), 0));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().0, TokenKind::INSTRUCTION(InstructionKind::ADDI));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::zero,  0));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::ra,   31));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::INTEGER(256));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().0, TokenKind::INSTRUCTION(InstructionKind::ADD));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::t1,  9));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::t2, 10));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::t3, 11));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().0, TokenKind::INSTRUCTION(InstructionKind::SUB));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::t4, 12));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::t5, 13));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::t6, 14));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().0, TokenKind::INSTRUCTION(InstructionKind::XOR));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().0, TokenKind::INSTRUCTION(InstructionKind::LI));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::v0, 2));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::INTEGER(1));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().0, TokenKind::INSTRUCTION(InstructionKind::MOVE));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::a0,  4));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::t2, 10));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().0, TokenKind::INSTRUCTION(InstructionKind::SYSCALL));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().0, TokenKind::INSTRUCTION(InstructionKind::SYSCALL));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().0, TokenKind::INSTRUCTION(InstructionKind::BLT));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::t0, 8));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::ADDRESS("label".to_string()));
    assert_eq!(tokens.consume().unwrap().0, TokenKind::EOL);

    // `cargo test -- --nocapture`
    tokens.reset();
    while let Some(token) = tokens.consume() {
        print!("{:?}", token);
        if token.0 == TokenKind::EOL {
            println!("");
        }
    }
}
