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

fn is_indicate(word: &str) -> bool {
    word.starts_with(".")
}

fn is_comment(word: &str) -> bool {
    word.starts_with("#")
}

/// Recieve 1 line
pub fn tokenize(number_of_lines: u32, line: &str, tokens: &mut Tokens) {
    let line = line.replace(",", " ");
    let words: Vec<&str> = line.split_whitespace().collect();

    // Skip blank line either comment line
    if words.len() == 0 || words.len() > 0 && words[0].starts_with("#") {
        return;
    }

    let mut words = words.iter();
    while let Some(word) = words.next() {
        if let Ok(num) = word.parse::<i32>() {
            tokens.push(TokenKind::INTEGER(num), number_of_lines);
        } else if let Ok((k, i)) = is_register(&word) {
            tokens.push(TokenKind::REGISTER(k, i), number_of_lines);
        } else if let Ok((k, i, a)) = is_stack(&word) {
            tokens.push(TokenKind::STACK(k, i, a), number_of_lines);
        } else if is_comment(&word) {
            break;
        } else {
            let token_kind = match &*word.to_ascii_uppercase() {
                // Arithmetic, Logic
                "ADD" |
                "ADDU"    => TokenKind::INSTRUCTION(InstructionKind::ADD),
                "ADDI" |
                "ADDIU"   => TokenKind::INSTRUCTION(InstructionKind::ADDI),
                "SUB" |
                "SUBU"    => TokenKind::INSTRUCTION(InstructionKind::SUB),
                "MUL"     => TokenKind::INSTRUCTION(InstructionKind::MUL),
                "DIV"     => TokenKind::INSTRUCTION(InstructionKind::DIV),
                "REM"     => TokenKind::INSTRUCTION(InstructionKind::REM),
                "AND"     => TokenKind::INSTRUCTION(InstructionKind::AND),
                "ANDI"    => TokenKind::INSTRUCTION(InstructionKind::ANDI),
                "OR"      => TokenKind::INSTRUCTION(InstructionKind::OR),
                "ORI"     => TokenKind::INSTRUCTION(InstructionKind::ORI),
                "XOR"     => TokenKind::INSTRUCTION(InstructionKind::XOR),
                "XORI"    => TokenKind::INSTRUCTION(InstructionKind::XORI),
                // Constant
                "LI"      => TokenKind::INSTRUCTION(InstructionKind::LI),
                "LUI"     => TokenKind::INSTRUCTION(InstructionKind::LUI),
                // Comparison
                "SLTU" |
                "SLT"     => TokenKind::INSTRUCTION(InstructionKind::SLT),
                "SLTIU" |
                "SLTI"    => TokenKind::INSTRUCTION(InstructionKind::SLTI),
                "SEQ"     => TokenKind::INSTRUCTION(InstructionKind::SEQ),
                "SGEU" |
                "SGE"     => TokenKind::INSTRUCTION(InstructionKind::SGE),
                "SGTU" |
                "SGT"     => TokenKind::INSTRUCTION(InstructionKind::SGT),
                "SLEU" |
                "SLE"     => TokenKind::INSTRUCTION(InstructionKind::SLE),
                "SNE"     => TokenKind::INSTRUCTION(InstructionKind::SNE),
                // Branch
                "B"       => TokenKind::INSTRUCTION(InstructionKind::B),
                "BEQ"     => TokenKind::INSTRUCTION(InstructionKind::BEQ),
                "BNE"     => TokenKind::INSTRUCTION(InstructionKind::BNE),
                "BGE" |
                "BGEU"    => TokenKind::INSTRUCTION(InstructionKind::BGE),
                "BGT" |
                "BGTU"    => TokenKind::INSTRUCTION(InstructionKind::BGT),
                "BLE" |
                "BLEU"    => TokenKind::INSTRUCTION(InstructionKind::BLE),
                "BLT" |
                "BLTU"    => TokenKind::INSTRUCTION(InstructionKind::BLT),
                "BEQZ"    => TokenKind::INSTRUCTION(InstructionKind::BEQZ),
                "BGEZ"    => TokenKind::INSTRUCTION(InstructionKind::BGEZ),
                "BGTZ"    => TokenKind::INSTRUCTION(InstructionKind::BGTZ),
                "BLEZ"    => TokenKind::INSTRUCTION(InstructionKind::BLEZ),
                "BLTZ"    => TokenKind::INSTRUCTION(InstructionKind::BLTZ),
                "BNEZ"    => TokenKind::INSTRUCTION(InstructionKind::BNEZ),
                // Jump
                "J"       => TokenKind::INSTRUCTION(InstructionKind::J),
                "JAL"     => TokenKind::INSTRUCTION(InstructionKind::JAL),
                "JR"      => TokenKind::INSTRUCTION(InstructionKind::JR),
                "JALR"    => TokenKind::INSTRUCTION(InstructionKind::JALR),
                // Load, Store
                "LA"      => TokenKind::INSTRUCTION(InstructionKind::LA),
                "LW"      => TokenKind::INSTRUCTION(InstructionKind::LW),
                "SW"      => TokenKind::INSTRUCTION(InstructionKind::SW),
                // Transfer
                "MOVE"    => TokenKind::INSTRUCTION(InstructionKind::MOVE),
                // Exception, Interrupt
                "SYSCALL" => TokenKind::INSTRUCTION(InstructionKind::SYSCALL),
                "NOP"     => TokenKind::INSTRUCTION(InstructionKind::NOP),
                _ =>
                    if is_label(&word) {
                        let mut identifier = word.to_string();
                        identifier.remove(identifier.len()-1);  // Delete ':'
                        TokenKind::LABEL(identifier, tokens.len())
                    } else if is_indicate(&word) {
                        match *word {
                            ".text" =>  TokenKind::INDICATE(IndicateKind::text),
                            ".data" =>  TokenKind::INDICATE(IndicateKind::data),
                            ".globl" => TokenKind::INDICATE(IndicateKind::globl),
                            ".word" => {
                                let num = words.next().unwrap().parse::<i32>().unwrap();
                                TokenKind::INDICATE(IndicateKind::word(num))
                            },
                            ".byte" => {
                                let ch = words.next().unwrap().parse::<char>().unwrap();
                                TokenKind::INDICATE(IndicateKind::byte(ch))
                            },
                            ".asciiz" => {
                                let mut s = words.next().unwrap().to_string();
                                s.remove(s.len()-1);  // Delete last  '"'
                                s.remove(0);          // Delete first '"'
                                TokenKind::INDICATE(IndicateKind::asciiz(s))
                            },
                            _ => TokenKind::INVALID(format!("invalid indicate: {}", word))
                        }
                    } else {
                        TokenKind::ADDRESS(word.to_string())
                    }
            };

            tokens.push(token_kind, number_of_lines);
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
    SLT SLTU SLTI SLTIU SEQ SGE SGEU SGT SGTU SLE SLEU SNE
    REM
    .text .data .globl
    .word 42 .byte a .asciiz \"string\"
";

    let mut tokens: Tokens = Tokens::new();
    let mut buf = String::new();
    let mut reader = BufReader::new(input.as_bytes());
    while reader.read_line(&mut buf).unwrap() > 0 {
        tokenize(0, &buf, &mut tokens);
        buf.clear();
    }

    assert_tokens(&mut tokens, TokenKind::LABEL("main".to_string(), 0));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::ADDI));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::zero,  0));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::ra,   31));
    assert_tokens(&mut tokens, TokenKind::INTEGER(256));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::ADD));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::t1,  9));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::t2, 10));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::t3, 11));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SUB));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::t4, 12));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::t5, 13));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::t6, 14));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::XOR));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::LI));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::v0, 2));
    assert_tokens(&mut tokens, TokenKind::INTEGER(1));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::MOVE));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::a0,  4));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::t2, 10));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SYSCALL));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SYSCALL));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BLT));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::t0, 8));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_tokens(&mut tokens, TokenKind::ADDRESS("label".to_string()));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::MUL));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::t4, 12));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::t5, 13));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::t6, 14));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::J));
    assert_tokens(&mut tokens, TokenKind::ADDRESS("hoge".to_string()));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::JAL));
    assert_tokens(&mut tokens, TokenKind::ADDRESS("fuga".to_string()));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::JR));
    assert_tokens(&mut tokens, TokenKind::REGISTER(RegisterKind::ra, 31));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::STACK(RegisterKind::sp, 29,  0));
    assert_tokens(&mut tokens, TokenKind::STACK(RegisterKind::t0,  8,  0));
    assert_tokens(&mut tokens, TokenKind::STACK(RegisterKind::t1,  9, 20));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::NOP));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::ADD));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::ADD));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::ADDI));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::ADDI));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SUB));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SUB));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::AND));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::ANDI));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::OR));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::ORI));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::XOR));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::XORI));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::B));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BEQ));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BNE));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BGE));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BGT));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BLE));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BLT));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BGE));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BGT));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BLE));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BLT));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BEQZ));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BGEZ));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BGTZ));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BLEZ));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BLTZ));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::BNEZ));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SLT));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SLT));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SLTI));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SLTI));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SEQ));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SGE));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SGE));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SGT));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SGT));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SLE));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SLE));
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::SNE));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INSTRUCTION(InstructionKind::REM));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INDICATE(IndicateKind::text));
    assert_tokens(&mut tokens, TokenKind::INDICATE(IndicateKind::data));
    assert_tokens(&mut tokens, TokenKind::INDICATE(IndicateKind::globl));
    assert_tokens(&mut tokens, TokenKind::EOL);
    assert_tokens(&mut tokens, TokenKind::INDICATE(IndicateKind::word(42)));
    assert_tokens(&mut tokens, TokenKind::INDICATE(IndicateKind::byte('a')));
    assert_tokens(&mut tokens, TokenKind::INDICATE(IndicateKind::asciiz("string".to_string())));
    assert_tokens(&mut tokens, TokenKind::EOL);

    // `cargo test -- --nocapture`
    tokens.reset();
    while let Some(token) = tokens.consume() {
        print!("{:?}", token);
        if token.kind == TokenKind::EOL {
            println!("");
        }
    }
}

#[cfg(test)]
fn assert_tokens(tokens: &mut Tokens, token_kind: TokenKind) {
    assert_eq!(tokens.consume().unwrap().kind, token_kind);
}

