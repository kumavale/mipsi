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
    let mut s = word.to_string();
    s.remove(s.len()-1);  // Delete ')'
    let mut s_chars = s.chars();
    while let Some(c) = s_chars.next() {
        let num = c as i32 - 48;
        if 0 <= num && num <= 9 {
            add = add * 10 + num;
        } else if c == '(' {
            let mut reg = String::new();
            #[allow(clippy::while_let_on_iterator)]
            while let Some(c) = s_chars.next() {
                reg = format!("{}{}", reg, c);
            }
            let (reg, idx) = is_register(&reg)?;
            return Ok((reg, idx, add));
        } else {
            break;
        }
    }
    Err(errmsg)
}

/// [a-zA-Z_][a-zA-Z_0-9]* \( `is_register` \)
fn is_data(word: &str) -> Result<(RegisterKind, usize, String), String> {
    let errmsg = format!("is_stack(): not stack identifier: {}", word);
    if Some(')') != word.chars().nth(word.len()-1) {
        return  Err(errmsg);
    }
    let mut s = word.to_string();
    s.remove(s.len()-1);  // Delete ')'
    let mut s_chars = s.chars();

    if let Some(c) = s_chars.next() {
        if 'A' <= c && c <= 'Z' || 'a' <= c && c <= 'z' || c == '_' {
            let mut label = c.to_string();

            while let Some(c) = s_chars.next() {
                if 'A' <= c && c <= 'Z' || 'a' <= c && c <= 'z' || c == '_' || '0' <= c && c <= '9' {
                    label = format!("{}{}", label, c);
                } else if c == '(' {
                    let mut reg = String::new();
                    #[allow(clippy::while_let_on_iterator)]
                    while let Some(c) = s_chars.next() {
                        reg = format!("{}{}", reg, c);
                    }
                    let (reg, idx) = is_register(&reg)?;
                    return Ok((reg, idx, label));
                } else {
                    break;
                }
            }
        }
    }
    Err(errmsg)
}

fn is_label(word: &str) -> bool {
    word.ends_with(':')
}

fn is_indicate(word: &str) -> bool {
    word.starts_with('.')
}

fn is_comment(word: &str) -> bool {
    word.starts_with('#')
}

fn is_hexadecimal(word: &str) -> Option<i32> {
    if word.starts_with("0x") {
        let mut hex: i32 = 0;
        let mut s = word.to_string();
        s.remove(0);
        s.remove(0);  // Delete "0x"

        for h in s.chars() {
            hex = match h {
                '0'..='9' => (hex << 4) + (h as i32 - 48),
                'a' | 'A' => (hex << 4) + 10,
                'b' | 'B' => (hex << 4) + 11,
                'c' | 'C' => (hex << 4) + 12,
                'd' | 'D' => (hex << 4) + 13,
                'e' | 'E' => (hex << 4) + 14,
                'f' | 'F' => (hex << 4) + 15,
                _ => return None,
            }
        }

        Some(hex)
    } else {
        None
    }
}

fn indicate_word(tokens: &mut Tokens, number_of_lines: u32, words: std::slice::Iter<&str>) {
    for word in words {
        let split: Vec<&str> = word.split(':').collect();
        if split.len() == 2 {
            for _ in 0..split[1].parse::<usize>().unwrap() {
                let num = {
                    if let Ok(num) = split[0].parse::<i32>() {
                        num as u32
                    } else {
                        split[0].parse::<u32>().unwrap()
                    }
                };
                tokens.push(TokenKind::INDICATE(IndicateKind::word(num)), number_of_lines);
            }
        } else if !is_comment(&word) {
            let num = {
                if let Ok(num) = split[0].parse::<i32>() {
                    num as u32
                } else {
                    split[0].parse::<u32>().unwrap()
                }
            };
            tokens.push(TokenKind::INDICATE(IndicateKind::word(num)), number_of_lines);
        } else {
            break;
        }
    };
}

fn indicate_half(tokens: &mut Tokens, number_of_lines: u32, words: std::slice::Iter<&str>) {
    for word in words {
        let split: Vec<&str> = word.split(':').collect();
        if split.len() == 2 {
            for _ in 0..split[1].parse::<usize>().unwrap() {
                let half = {
                    if let Ok(num) = split[0].parse::<i16>() {
                        num as u16
                    } else {
                        split[0].parse::<u16>().unwrap()
                    }
                };
                tokens.push(TokenKind::INDICATE(IndicateKind::half(half)), number_of_lines);
            }
        } else if !is_comment(&word) {
            let half = {
                if let Ok(num) = split[0].parse::<i16>() {
                    num as u16
                } else {
                    split[0].parse::<u16>().unwrap()
                }
            };
            tokens.push(TokenKind::INDICATE(IndicateKind::half(half)), number_of_lines);
        } else {
            break;
        }
    };
}

fn indicate_byte(tokens: &mut Tokens, number_of_lines: u32, words: std::slice::Iter<&str>) {
    let mut byte = 0;
    for word in words {
        let split: Vec<&str> = word.split(':').collect();
        if split.len() == 2 {
            for _ in 0..split[1].parse::<usize>().unwrap() {
                byte = {
                    if let Ok(num) = split[0].parse::<i8>() {
                        num as u8
                    } else {
                        split[0].parse::<u8>().unwrap()
                    }
                };
                tokens.push(TokenKind::INDICATE(IndicateKind::byte(byte)), number_of_lines);
            }
        } else if word.starts_with(':') {
            let mut word = word.to_string();
            word.remove(0);
            for _ in 1..word.parse::<usize>().unwrap() {
                tokens.push(TokenKind::INDICATE(IndicateKind::byte(byte)), number_of_lines);
            }
        } else if !is_comment(&word) {
            byte = {
                if let Ok(num) = split[0].parse::<i8>() {
                    num as u8
                } else {
                    split[0].parse::<u8>().unwrap()
                }
            };
            tokens.push(TokenKind::INDICATE(IndicateKind::byte(byte)), number_of_lines);
        } else {
            break;
        }
    };
}

fn split_words(line: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut line_iter = line.chars();

    while let Some(ch) = line_iter.next() {
        // Skip white space
        match ch {
            ' ' | ',' | '\n' | '\r' | '\t' => continue,
            _ => (),
        }

        // string for .asciiz | literal
        if ch == '"' {
            let mut asciiz = "\"".to_string();
            while let Some(mut ch2) = line_iter.next() {
                if ch2 != '"' {
                    if ch2 == '\\' {
                        let ch3 = line_iter.next().unwrap();
                        ch2 = match ch3 {
                            '\\' => '\\',
                            '\'' => '\'',
                            '"'  => '\"',
                            '0'  => '\0',
                            'n'  => '\n',
                            'r'  => '\r',
                            't'  => '\t',
                            _ => panic!("not support this escape sequence: \\{}", ch3),
                        };
                    }
                    asciiz = format!("{}{}", asciiz, ch2);
                    continue;
                }
                asciiz.push('"');
                break;
            }
            words.push(asciiz);

        // char for .byte
        // char to ascii code (e.g. 'a'=>97)
        } else if ch == '\'' {
            let byte = line_iter.next().unwrap();
            if byte == '\\' {
                let ch2 = line_iter.next().unwrap();
                let byte = match ch2 {
                    '\\' => 92,
                    '\'' => 39,
                    '"'  => 34,
                    '0'  =>  0,
                    'n'  => 10,
                    'r'  => 13,
                    't'  =>  9,
                    _ => panic!("not support this escape sequence: \\{}", ch2),
                };
                let ch2 = line_iter.next().unwrap();
                // expect '\''
                if ch2 != '\'' {
                    panic!(".byte: not 1-byte");
                }
                words.push(byte.to_string());
            } else if byte == '\'' {
                words.push(0.to_string());
            } else {
                let ch2 = line_iter.next().unwrap();
                // expect '\''
                if ch2 != '\'' {
                    panic!(".byte: not 1-byte");
                }
                words.push((byte as u8).to_string());
            }

        // word except string
        } else {
            let mut word = format!("{}", ch);
            #[allow(clippy::while_let_on_iterator)]
            while let Some(ch2) = line_iter.next() {
                match ch2 {
                    ' ' | ',' | '\n' | '\r' | '\t' => { break; },
                    _ => {
                        word = format!("{}{}", word, ch2);
                    },
                }
            }
            words.push(word);
        }
    }

    words
}

/// Recieve 1 line
pub fn tokenize(number_of_lines: u32, line: &str, mut tokens: &mut Tokens) {
    let words: Vec<String> = split_words(&line);
    let words: Vec<&str>   = words.iter().map(|s| &**s).collect();

    //println!("{:?}", words);

    // Skip blank line either comment line
    if words.is_empty() || !words.is_empty() && words[0].starts_with('#') {
        return;
    }

    let mut words = words.iter();
    while let Some(word) = words.next() {
        if let Ok(num) = word.parse::<i32>() {
            tokens.push(TokenKind::INTEGER(num), number_of_lines);
        } else if let Some(num) = is_hexadecimal(&word) {
            tokens.push(TokenKind::INTEGER(num), number_of_lines);
        } else if let Ok((k, i)) = is_register(&word) {
            tokens.push(TokenKind::REGISTER(k, i), number_of_lines);
        } else if let Ok((k, i, a)) = is_stack(&word) {
            tokens.push(TokenKind::STACK(k, i, a), number_of_lines);
        } else if let Ok((k, i, s)) = is_data(&word) {
            tokens.push(TokenKind::DATA(k, i, s), number_of_lines);
        } else if is_comment(&word) {
            break;
        } else {
            let token_kind = match &*word.to_ascii_uppercase() {
                // Arithmetic, Logic
                "ADD"     => TokenKind::INSTRUCTION(InstructionKind::ADD),
                "ADDU"    => TokenKind::INSTRUCTION(InstructionKind::ADDU),
                "ADDI"    => TokenKind::INSTRUCTION(InstructionKind::ADDI),
                "ADDIU"   => TokenKind::INSTRUCTION(InstructionKind::ADDIU),
                "SUB"     => TokenKind::INSTRUCTION(InstructionKind::SUB),
                "SUBU"    => TokenKind::INSTRUCTION(InstructionKind::SUBU),
                "MUL"     => TokenKind::INSTRUCTION(InstructionKind::MUL),
                "REM"     => TokenKind::INSTRUCTION(InstructionKind::REM),
                "REMU"    => TokenKind::INSTRUCTION(InstructionKind::REMU),

                "MULO"    => TokenKind::INSTRUCTION(InstructionKind::MULO),
                "MULOU"   => TokenKind::INSTRUCTION(InstructionKind::MULOU),
                "CLO"     => TokenKind::INSTRUCTION(InstructionKind::CLO),
                "CLZ"     => TokenKind::INSTRUCTION(InstructionKind::CLZ),
                "ROR"     => TokenKind::INSTRUCTION(InstructionKind::ROR),
                "ROL"     => TokenKind::INSTRUCTION(InstructionKind::ROL),

                "DIV"     => TokenKind::INSTRUCTION(InstructionKind::DIV),
                "DIVU"    => TokenKind::INSTRUCTION(InstructionKind::DIVU),
                "MULT"    => TokenKind::INSTRUCTION(InstructionKind::MULT),
                "MULTU"   => TokenKind::INSTRUCTION(InstructionKind::MULTU),
                "MADD"    => TokenKind::INSTRUCTION(InstructionKind::MADD),
                "MADDU"   => TokenKind::INSTRUCTION(InstructionKind::MADDU),
                "MSUB"    => TokenKind::INSTRUCTION(InstructionKind::MSUB),
                "MSUBU"   => TokenKind::INSTRUCTION(InstructionKind::MSUBU),

                "NOR"     => TokenKind::INSTRUCTION(InstructionKind::NOR),
                "NOT"     => TokenKind::INSTRUCTION(InstructionKind::NOT),
                "NEG"     => TokenKind::INSTRUCTION(InstructionKind::NEG),
                "NEGU"    => TokenKind::INSTRUCTION(InstructionKind::NEGU),

                "SLL"     => TokenKind::INSTRUCTION(InstructionKind::SLL),
                "SLLV"    => TokenKind::INSTRUCTION(InstructionKind::SLLV),
                "SRA"     => TokenKind::INSTRUCTION(InstructionKind::SRA),
                "SRAV"    => TokenKind::INSTRUCTION(InstructionKind::SRAV),
                "SRL"     => TokenKind::INSTRUCTION(InstructionKind::SRL),
                "SRLV"    => TokenKind::INSTRUCTION(InstructionKind::SRLV),

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
                "LB"      => TokenKind::INSTRUCTION(InstructionKind::LB),
                "LH"      => TokenKind::INSTRUCTION(InstructionKind::LH),
                "LW"      => TokenKind::INSTRUCTION(InstructionKind::LW),
                "SW"      => TokenKind::INSTRUCTION(InstructionKind::SW),

                // Transfer
                "MOVE"    => TokenKind::INSTRUCTION(InstructionKind::MOVE),

                // Exception, Interrupt
                "SYSCALL" => TokenKind::INSTRUCTION(InstructionKind::SYSCALL),
                "NOP"     => TokenKind::INSTRUCTION(InstructionKind::NOP),

                // My own
                "PRTN"    => TokenKind::INSTRUCTION(InstructionKind::PRTN),
                "PRTI"    => TokenKind::INSTRUCTION(InstructionKind::PRTI),
                "PRTH"    => TokenKind::INSTRUCTION(InstructionKind::PRTH),
                "PRTX"    => TokenKind::INSTRUCTION(InstructionKind::PRTX),
                "PRTC"    => TokenKind::INSTRUCTION(InstructionKind::PRTC),
                "PRTS"    => TokenKind::INSTRUCTION(InstructionKind::PRTS),

                _ =>
                    if is_label(&word) {
                        let mut identifier = word.to_string();
                        identifier.remove(identifier.len()-1);  // Delete ':'
                        TokenKind::LABEL(identifier, tokens.len(), None)
                    } else if is_indicate(&word) {
                        match *word {
                            // TODO
                            // unwrap() => TokenKind::INVALID()
                            ".text" =>  TokenKind::INDICATE(IndicateKind::text),
                            ".data" =>  TokenKind::INDICATE(IndicateKind::data),
                            ".globl" => {
                                let label = words.next().unwrap().to_string();
                                TokenKind::INDICATE(IndicateKind::globl(label))
                            },
                            ".word" => {
                                indicate_word(&mut tokens, number_of_lines, words);
                                break;
                            },
                            ".half" => {
                                indicate_half(&mut tokens, number_of_lines, words);
                                break;
                            },
                            ".byte" => {
                                indicate_byte(&mut tokens, number_of_lines, words);
                                break;
                            },
                            ".space" => {
                                let length = words.next().unwrap().parse::<u32>().unwrap();
                                TokenKind::INDICATE(IndicateKind::space(length))
                            },
                            ".ascii" => {
                                let mut s = words.next().unwrap().to_string();
                                s.remove(0);
                                s.remove(s.len()-1);
                                TokenKind::INDICATE(IndicateKind::ascii(s))
                            },
                            ".asciiz" => {
                                let mut s = words.next().unwrap().to_string();
                                s.remove(0);
                                s.remove(s.len()-1);
                                TokenKind::INDICATE(IndicateKind::asciiz(s))
                            },
                            ".align" => {
                                let n = words.next().unwrap().parse::<u8>().unwrap();
                                TokenKind::INDICATE(IndicateKind::align(n))
                            },
                            _ => TokenKind::INVALID(format!("invalid indicate: {}", word))
                        }
                    } else  if word.starts_with('"')  && word.ends_with('"') ||
                               word.starts_with('\'') && word.ends_with('\'') {
                        let mut word = word.to_string();
                        word.remove(0);
                        word.remove(word.len()-1);
                        TokenKind::LITERAL(word)
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
#[allow(clippy::cognitive_complexity)]
fn test_tokenize() {
    use std::io::{BufRead, BufReader, Write};

    let input = "\
# This is comment.

\n
\t
\r
  
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
    REM REMU
    .text .data .globl main
w:  .word 42, 0, 1, 2, 3
h:  .half 3, 2, 1, 0, 42
b:  .byte 'a', 'i', 'u', 'e', 'o'
s:  .ascii \"string\"
z:  .asciiz \"stringz\"
n:  .space 256
    NOR NOT
    SLL SLLV SRA SRAV SRL SRLV
    LB LH LW
    .ascii \"string\"
    .align 2

\t\r\"literal\"\n
\"\tstr\\n\"
'C' 'h' 'a' 'r'
'\\n','\\r','\\t','\\0'
'\\'' '\\\"'

NEG NEGU SW REMU MULO MULOU
CLO CLZ ROR ROL
DIV DIVU MULT MULTU MADD MADDU MSUB MSUBU
PRTN PRTI PRTH PRTX PRTC PRTS

.word 0:0
.word 1:1
.half 2:2
.byte 4:4

.word  -2147483648, -1, 0, 1, 2147483647, 4294967295
.half  -32768, -1, 0, 1, 32767, 65535
.byte  -128, -1, -0, 0, 1, 127, 255
";

    let mut tokens: Tokens = Tokens::new();
    let mut buf = String::new();
    let mut reader = BufReader::new(input.as_bytes());
    while reader.read_line(&mut buf).unwrap() > 0 {
        tokenize(0, &buf, &mut tokens);
        buf.clear();
    }

    // `cargo test -- --nocapture`
    while let Some(token) = tokens.consume() {
        print!("{:?}", token);
        std::io::stdout().flush().unwrap();
        if token.kind == TokenKind::EOL {
            println!();
        }
    }
    tokens.reset();

    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("main".to_string(), 0, None));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ADDI));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::zero,  0));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::ra,   31));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(256));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ADD));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t1,  9));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t2, 10));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t3, 11));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SUB));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t4, 12));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t5, 13));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t6, 14));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::XOR));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::LI));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::v0, 2));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(1));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MOVE));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::a0,  4));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t2, 10));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SYSCALL));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SYSCALL));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BLT));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t0, 8));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t1, 9));
    assert_eq!(tokens.consume_kind(), TokenKind::ADDRESS("label".to_string()));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MUL));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t4, 12));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t5, 13));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::t6, 14));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::J));
    assert_eq!(tokens.consume_kind(), TokenKind::ADDRESS("hoge".to_string()));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::JAL));
    assert_eq!(tokens.consume_kind(), TokenKind::ADDRESS("fuga".to_string()));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::JR));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::ra, 31));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::STACK(RegisterKind::sp, 29,  0));
    assert_eq!(tokens.consume_kind(), TokenKind::STACK(RegisterKind::t0,  8,  0));
    assert_eq!(tokens.consume_kind(), TokenKind::STACK(RegisterKind::t1,  9, 20));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::NOP));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ADD));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ADDU));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ADDI));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ADDIU));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SUB));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SUBU));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::AND));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ANDI));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::OR));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ORI));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::XOR));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::XORI));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::B));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BEQ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BNE));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BGE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BGT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BLE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BLT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BGE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BGT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BLE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BLT));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BEQZ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BGEZ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BGTZ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BLEZ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BLTZ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::BNEZ));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLTI));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLTI));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SEQ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SGE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SGE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SGT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SGT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLE));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SNE));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::REM));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::REMU));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::text));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::data));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::globl("main".to_string())));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("w".to_string(), 113, None));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(42)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(0)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(1)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(2)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(3)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("h".to_string(), 120, None));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(3)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(2)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(1)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(0)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(42)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("b".to_string(), 127, None));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(97)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(105)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(117)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(101)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(111)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("s".to_string(), 134, None));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::ascii("string".to_string())));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("z".to_string(), 137, None));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::asciiz("stringz".to_string())));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("n".to_string(), 140, None));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::space(256)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::NOR));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::NOT));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLL));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SLLV));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SRA));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SRAV));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SRL));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SRLV));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::LB));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::LH));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::LW));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::ascii("string".to_string())));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::align(2)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LITERAL("literal".to_string()));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LITERAL("\tstr\n".to_string()));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(67));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(104));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(97));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(114));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(10));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(13));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(9));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(0));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(39));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(34));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::NEG));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::NEGU));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SW));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::REMU));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MULO));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MULOU));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::CLO));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::CLZ));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ROR));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::ROL));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::DIV));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::DIVU));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MULT));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MULTU));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MADD));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MADDU));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MSUB));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::MSUBU));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::PRTN));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::PRTI));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::PRTH));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::PRTX));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::PRTC));
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::PRTS));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(1)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(2)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(2)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(4)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(4)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(4)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(4)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(2147483648)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(4294967295)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(0)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(1)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(2147483647)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::word(4294967295)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(32768)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(65535)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(0)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(1)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(32767)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::half(65535)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(128)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(255)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(0)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(0)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(1)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(127)));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::byte(255)));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
}

#[test]
#[cfg(test)]
fn test_tokenize_hello_world() {
    use std::io::{BufRead, BufReader};

    let input = "\
# Hello, World!\n
.data ## Data declaration section\n
## String to be printed:\n
out_string: .asciiz \"Hello, World!\\n\"\n
.text ## Assembly language instructions go in text segment\n
main: ## Start of code section\n
    li $v0, 4           # system call code for printing string = 4\n
    la $a0, out_string  # load address of string to be printed into $a0\n
    syscall             # call operating system to perform operation\n
                        # specified in $v0\n
                        # syscall takes its arguments from $a0, $a1, ...\n
    li $v0, 10          # terminate program\n
    syscall\n
";

    let mut tokens: Tokens = Tokens::new();
    let mut buf = String::new();
    let mut reader = BufReader::new(input.as_bytes());
    while reader.read_line(&mut buf).unwrap() > 0 {
        tokenize(0, &buf, &mut tokens);
        buf.clear();
    }

    // Hello World
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::data));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("out_string".to_string(), 2, None));
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::asciiz("Hello, World!\n".to_string())));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INDICATE(IndicateKind::text));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::LABEL("main".to_string(), 7, None));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::LI));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::v0, 2));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(4));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::LA));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::a0, 4));
    assert_eq!(tokens.consume_kind(), TokenKind::ADDRESS("out_string".to_string()));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SYSCALL));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::LI));
    assert_eq!(tokens.consume_kind(), TokenKind::REGISTER(RegisterKind::v0, 2));
    assert_eq!(tokens.consume_kind(), TokenKind::INTEGER(10));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
    assert_eq!(tokens.consume_kind(), TokenKind::INSTRUCTION(InstructionKind::SYSCALL));
    assert_eq!(tokens.consume_kind(), TokenKind::EOL);
}

#[cfg(test)]
impl Tokens {
    pub fn consume_kind(&mut self) -> TokenKind {
        self.consume().unwrap().kind
    }
}

