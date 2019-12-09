use super::token::*;

fn is_register(word: &str) -> Result<(RegisterKind, usize), String> {
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
        _ => return Err(format!("is_register(): invalid register name: {}", word)),
    };

    Ok((register_kind, idx))
}

/// [0-9]?[0-9]* \( `is_register` \)
fn is_stack(word: &str) -> Result<(RegisterKind, usize, i32), String> {
    let errmsg = format!("is_stack(): not stack identifier: {}", word);
    if Some(')') != word.chars().nth(word.len()-1) {
        return  Err(errmsg);
    }
    let mut add = 0;
    let mut s = word.clone().to_string();
    s.remove(s.len()-1);  // Delete ')'
    let mut s_chars = s.chars();
    while let Some(c) = s_chars.next() {
        let num = c as i32 - 48;
        if 0 <= num && num <= 9 {
            add = add * 10 + num;
        } else {
            if c == '(' {
                let mut reg = String::new();
                while let Some(c) = s_chars.next() {
                    reg = format!("{}{}", reg, c);
                }
                let (reg, idx) = is_register(&reg)?;
                return Ok((reg, idx, add));
            } else {
                break;
            }
        }
    }
    Err(errmsg)
}

fn is_label(word: &str) -> bool {
    word.ends_with(":")
}

fn is_comment(word: &str) -> bool {
    word.starts_with("#")
}

pub fn tokenize(number_of_lines: u32, line: &str, tokens: &mut Tokens) {
    let line = line.replace(",", " ");
    let words: Vec<&str> = line.split_whitespace().collect();

    // Skip blank line either comment line
    if words.len() == 0 || words.len() > 0 && words[0].starts_with("#") {
        return;
    }

    for word in words {
        if let Ok(num) = word.parse::<i32>() {
            tokens.push(TokenKind::INTEGER(num), number_of_lines);
        } else if let Ok((k, i)) = is_register(&word) {
            tokens.push(TokenKind::REGISTER(k, i), number_of_lines);
        } else if let Ok((k, i, a)) = is_stack(&word) {
            tokens.push(TokenKind::STACK(k, i, a), number_of_lines);
        } else if is_comment(&word) {
            break;
        } else {
            match &*word.to_ascii_uppercase() {
                // Arithmetic, Logic
                "ADD" |
                "ADDU"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::ADD),     number_of_lines),
                "ADDI" |
                "ADDIU"   => tokens.push(TokenKind::INSTRUCTION(InstructionKind::ADDI),    number_of_lines),
                "SUB" |
                "SUBU"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::SUB),     number_of_lines),
                "MUL"     => tokens.push(TokenKind::INSTRUCTION(InstructionKind::MUL),     number_of_lines),
                "DIV"     => tokens.push(TokenKind::INSTRUCTION(InstructionKind::DIV),     number_of_lines),
                "AND"     => tokens.push(TokenKind::INSTRUCTION(InstructionKind::AND),     number_of_lines),
                "ANDI"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::ANDI),    number_of_lines),
                "OR"      => tokens.push(TokenKind::INSTRUCTION(InstructionKind::OR),      number_of_lines),
                "ORI"     => tokens.push(TokenKind::INSTRUCTION(InstructionKind::ORI),     number_of_lines),
                "XOR"     => tokens.push(TokenKind::INSTRUCTION(InstructionKind::XOR),     number_of_lines),
                "XORI"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::XORI),    number_of_lines),
                // Constant
                "LI"      => tokens.push(TokenKind::INSTRUCTION(InstructionKind::LI),      number_of_lines),
                "LUI"     => tokens.push(TokenKind::INSTRUCTION(InstructionKind::LUI),     number_of_lines),
                // Comparison
                // Branch
                "B"       => tokens.push(TokenKind::INSTRUCTION(InstructionKind::B),       number_of_lines),
                "BEQ"     => tokens.push(TokenKind::INSTRUCTION(InstructionKind::BEQ),     number_of_lines),
                "BNE"     => tokens.push(TokenKind::INSTRUCTION(InstructionKind::BNE),     number_of_lines),
                "BGE" |
                "BGEU"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::BGE),     number_of_lines),
                "BGT" |
                "BGTU"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::BGT),     number_of_lines),
                "BLE" |
                "BLEU"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::BLE),     number_of_lines),
                "BLT" |
                "BLTU"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::BLT),     number_of_lines),
                "BEQZ"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::BEQZ),    number_of_lines),
                "BGEZ"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::BGEZ),    number_of_lines),
                "BGTZ"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::BGTZ),    number_of_lines),
                "BLEZ"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::BLEZ),    number_of_lines),
                "BLTZ"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::BLTZ),    number_of_lines),
                "BNEZ"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::BNEZ),    number_of_lines),
                // Jump
                "J"       => tokens.push(TokenKind::INSTRUCTION(InstructionKind::J),       number_of_lines),
                "JAL"     => tokens.push(TokenKind::INSTRUCTION(InstructionKind::JAL),     number_of_lines),
                "JR"      => tokens.push(TokenKind::INSTRUCTION(InstructionKind::JR),      number_of_lines),
                "JALR"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::JALR),    number_of_lines),
                // Load, Store
                "LA"      => tokens.push(TokenKind::INSTRUCTION(InstructionKind::LA),      number_of_lines),
                "LW"      => tokens.push(TokenKind::INSTRUCTION(InstructionKind::LW),      number_of_lines),
                "SW"      => tokens.push(TokenKind::INSTRUCTION(InstructionKind::SW),      number_of_lines),
                // Transfer
                "MOVE"    => tokens.push(TokenKind::INSTRUCTION(InstructionKind::MOVE),    number_of_lines),
                // Exception, Interrupt
                "SYSCALL" => tokens.push(TokenKind::INSTRUCTION(InstructionKind::SYSCALL), number_of_lines),
                "NOP"     => tokens.push(TokenKind::INSTRUCTION(InstructionKind::NOP),     number_of_lines),
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
    BLT     $t0,$t1,label
    mul     $t4,$t5,$t6
    J       hoge
    JAL     fuga
    JR      $ra
    ($sp)   0($t0)  20($t1)
    ##### SYSCALL ##### J J J
    NOP
    ADD ADDU ADDI ADDIU SUB SUBU
    AND ANDI OR ORI XOR XORI
    B BEQ BNE
    BGE BGT BLE BLT BGEU BGTU BLEU BLTU
    BEQZ BGEZ BGTZ BLEZ BLTZ BNEZ
";

    let mut tokens: Tokens = Tokens::new();
    let mut buf = String::new();
    let mut reader = BufReader::new(input.as_bytes());
    while reader.read_line(&mut buf).unwrap() > 0 {
        tokenize(0, &buf, &mut tokens);
        buf.clear();
    }

    assert_eq!(tokens.consume().unwrap().kind, TokenKind::LABEL("main".to_string(), 0));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::ADDI));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::zero,  0));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::ra,   31));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INTEGER(256));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::ADD));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::t1,  9));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::t2, 10));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::t3, 11));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::SUB));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::t4, 12));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::t5, 13));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::t6, 14));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::XOR));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::LI));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::v0, 2));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INTEGER(1));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::MOVE));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::a0,  4));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::t2, 10));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::SYSCALL));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::SYSCALL));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BLT));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::t0, 8));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::ADDRESS("label".to_string()));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::MUL));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::t4, 12));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::t5, 13));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::t6, 14));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::J));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::ADDRESS("hoge".to_string()));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::JAL));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::ADDRESS("fuga".to_string()));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::JR));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::REGISTER(RegisterKind::ra, 31));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::STACK(RegisterKind::sp, 29,  0));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::STACK(RegisterKind::t0,  8,  0));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::STACK(RegisterKind::t1,  9, 20));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::NOP));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::ADD));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::ADD));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::ADDI));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::ADDI));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::SUB));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::SUB));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::AND));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::ANDI));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::OR));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::ORI));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::XOR));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::XORI));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::B));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BEQ));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BNE));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BGE));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BGT));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BLE));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BLT));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BGE));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BGT));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BLE));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BLT));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BEQZ));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BGEZ));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BGTZ));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BLEZ));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BLTZ));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::INSTRUCTION(InstructionKind::BNEZ));
    assert_eq!(tokens.consume().unwrap().kind, TokenKind::EOL);

    // `cargo test -- --nocapture`
    tokens.reset();
    while let Some(token) = tokens.consume() {
        print!("{:?}", token);
        if token.kind == TokenKind::EOL {
            println!("");
        }
    }
}
