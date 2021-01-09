mod test;
use super::token::*;
use super::token::register::RegisterKind;

/// Recieve 1 line
/// nol: number_of_lines
/// fi:  filename_idx
pub fn tokenize(nol: u32, fi: usize, line: &str, mut tokens: &mut Tokens)
    -> Result<(), String>
{
    let words: Vec<String> = split_words(&line);
    let words: Vec<&str>   = words.iter().map(|s| &**s).collect();

    //println!("{:?}", words);

    // Ignore blank line
    if words.is_empty() {
        return Ok(());
    }

    let mut words = words.iter();
    while let Some(word) = words.next() {
        if let Ok(num) = word.parse::<i32>() {
            tokens.push(TokenKind::INTEGER(num), nol, fi);
        } else if let Ok(num) = word.parse::<f32>() {
            tokens.push(TokenKind::FLOATING(num), nol, fi);
        } else if let Some(num) = is_hexadecimal(&word) {
            tokens.push(TokenKind::INTEGER(num), nol, fi);
        } else if let Ok((k, i)) = is_register(&word) {
            tokens.push(TokenKind::REGISTER(k, i), nol, fi);
        } else if let Ok((k, i, a)) = is_memory(&word) {
            tokens.push(TokenKind::MEMORY(k, i, a), nol, fi);
        } else if let Ok((k, i, s)) = is_data_address(&word) {
            tokens.push(TokenKind::DATA(k, i, s), nol, fi);
        } else {
            let token_kind = match &*word.to_ascii_uppercase() {
                // CPU Instructions
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
                "BGEZAL"  => TokenKind::INSTRUCTION(InstructionKind::BGEZAL),
                "BLTZAL"  => TokenKind::INSTRUCTION(InstructionKind::BLTZAL),

                // Jump
                "J"       => TokenKind::INSTRUCTION(InstructionKind::J),
                "JAL"     => TokenKind::INSTRUCTION(InstructionKind::JAL),
                "JR"      => TokenKind::INSTRUCTION(InstructionKind::JR),
                "JALR"    => TokenKind::INSTRUCTION(InstructionKind::JALR),

                // Load
                "LA"      => TokenKind::INSTRUCTION(InstructionKind::LA),
                "LB"      => TokenKind::INSTRUCTION(InstructionKind::LB),
                "LBU"     => TokenKind::INSTRUCTION(InstructionKind::LBU),
                "LH"      => TokenKind::INSTRUCTION(InstructionKind::LH),
                "LHU"     => TokenKind::INSTRUCTION(InstructionKind::LHU),
                "LW"      => TokenKind::INSTRUCTION(InstructionKind::LW),

                // Store
                "SB"      => TokenKind::INSTRUCTION(InstructionKind::SB),
                "SH"      => TokenKind::INSTRUCTION(InstructionKind::SH),
                "SW"      => TokenKind::INSTRUCTION(InstructionKind::SW),

                // Transfer
                "MOVE"    => TokenKind::INSTRUCTION(InstructionKind::MOVE),
                "MFHI"    => TokenKind::INSTRUCTION(InstructionKind::MFHI),
                "MFLO"    => TokenKind::INSTRUCTION(InstructionKind::MFLO),
                "MTHI"    => TokenKind::INSTRUCTION(InstructionKind::MTHI),
                "MTLO"    => TokenKind::INSTRUCTION(InstructionKind::MTLO),
                "MOVN"    => TokenKind::INSTRUCTION(InstructionKind::MOVN),
                "MOVZ"    => TokenKind::INSTRUCTION(InstructionKind::MOVZ),

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
                "RST"     => TokenKind::INSTRUCTION(InstructionKind::RST),

                // FPU Instructions
                "L.S"     => TokenKind::INSTRUCTION(InstructionKind::LA),
                "LI.S"    => TokenKind::INSTRUCTION(InstructionKind::LI),
                "LWC1"    => TokenKind::INSTRUCTION(InstructionKind::LW),
                "SWC1"    => TokenKind::INSTRUCTION(InstructionKind::SW),
                "MTC1"    => TokenKind::INSTRUCTION(InstructionKind::MTC1),
                "ABS.S"   => TokenKind::INSTRUCTION(InstructionKind::ABS_S),
                "ADD.S"   => TokenKind::INSTRUCTION(InstructionKind::ADD_S),
                "DIV.S"   => TokenKind::INSTRUCTION(InstructionKind::DIV_S),
                "MUL.S"   => TokenKind::INSTRUCTION(InstructionKind::MUL_S),
                "SUB.S"   => TokenKind::INSTRUCTION(InstructionKind::SUB_S),
                "NEG.S"   => TokenKind::INSTRUCTION(InstructionKind::NEG_S),
                "BC1T"    => TokenKind::INSTRUCTION(InstructionKind::BC1T),
                "BC1F"    => TokenKind::INSTRUCTION(InstructionKind::BC1F),
                "C.EQ.S"  => TokenKind::INSTRUCTION(InstructionKind::C_EQ_S),
                "C.LE.S"  => TokenKind::INSTRUCTION(InstructionKind::C_LE_S),
                "C.LT.S"  => TokenKind::INSTRUCTION(InstructionKind::C_LT_S),
                "CVT.S.W" => TokenKind::INSTRUCTION(InstructionKind::CVT_S_W),
                "CVT.W.S" => TokenKind::INSTRUCTION(InstructionKind::CVT_W_S),

                _ =>
                    if is_label(&word) {
                        let mut identifier = (*word).to_string();
                        identifier.pop();  // Delete ':'
                        tokens.add_address(identifier.clone(), tokens.len());
                        TokenKind::LABEL(identifier, tokens.len(), None)
                    } else if is_indicate(&word) {
                        match *word {
                            ".text" => {
                                // ignore <Addr>
                                let _ = words.next();
                                TokenKind::INDICATE(IndicateKind::text)
                            },
                            ".data" => {
                                // ignore <Addr>
                                let _ = words.next();
                                TokenKind::INDICATE(IndicateKind::data)
                            },
                            ".globl" => {
                                let label = (*words.next().unwrap()).to_string();
                                TokenKind::INDICATE(IndicateKind::globl(label))
                            },
                            ".word" => {
                                indicate_word(&mut tokens, nol, fi, words);
                                break;
                            },
                            ".half" => {
                                indicate_half(&mut tokens, nol, fi, words);
                                break;
                            },
                            ".byte" => {
                                indicate_byte(&mut tokens, nol, fi, words);
                                break;
                            },
                            ".float" => {
                                indicate_float(&mut tokens, nol, fi, words);
                                break;
                            },
                            ".space" => {
                                let word = words.next().unwrap();
                                let length = indicate_space(&word);
                                TokenKind::INDICATE(IndicateKind::space(length))
                            },
                            ".ascii" => {
                                let mut s = (*words.next().unwrap()).to_string();
                                s.remove(0);
                                s.remove(s.len()-1);
                                TokenKind::INDICATE(IndicateKind::ascii(s))
                            },
                            ".asciiz" => {
                                let mut s = (*words.next().unwrap()).to_string();
                                s.remove(0);
                                s.remove(s.len()-1);
                                TokenKind::INDICATE(IndicateKind::asciiz(s))
                            },
                            ".align" => {
                                let n = words.next().unwrap().parse::<u8>().unwrap();
                                TokenKind::INDICATE(IndicateKind::align(n))
                            },
                            _ => return Err(format!("invalid indicate: {}", word)),
                        }
                    } else  if word.starts_with('"')  && word.ends_with('"') ||
                               word.starts_with('\'') && word.ends_with('\'') {
                        let mut word = (*word).to_string();
                        word.remove(0);
                        word.remove(word.len()-1);
                        TokenKind::LITERAL(word)
                    } else {
                        TokenKind::ADDRESS((*word).to_string())
                    }
            };

            tokens.push(token_kind, nol, fi);
        }
    }

    tokens.push(TokenKind::EOL, nol, fi);
    Ok(())
}


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

        "fv0" | "f0"   => (RegisterKind::f0,   32),
                "f1"   => (RegisterKind::f1,   33),
        "fv1" | "f2"   => (RegisterKind::f2,   34),
                "f3"   => (RegisterKind::f3,   35),
        "ft0" | "f4"   => (RegisterKind::f4,   36),
                "f5"   => (RegisterKind::f5,   37),
        "ft1" | "f6"   => (RegisterKind::f6,   38),
                "f7"   => (RegisterKind::f7,   39),
        "ft2" | "f8"   => (RegisterKind::f8,   40),
                "f9"   => (RegisterKind::f9,   41),
        "ft3" | "f10"  => (RegisterKind::f10,  42),
                "f11"  => (RegisterKind::f11,  43),
        "fa0" | "f12"  => (RegisterKind::f12,  44),
                "f13"  => (RegisterKind::f13,  45),
        "fa1" | "f14"  => (RegisterKind::f14,  46),
                "f15"  => (RegisterKind::f15,  47),
        "ft4" | "f16"  => (RegisterKind::f16,  48),
                "f17"  => (RegisterKind::f17,  49),
        "ft8" | "f18"  => (RegisterKind::f18,  50),
                "f19"  => (RegisterKind::f19,  51),
        "fs0" | "f20"  => (RegisterKind::f20,  52),
                "f21"  => (RegisterKind::f21,  53),
        "fs1" | "f22"  => (RegisterKind::f22,  54),
                "f23"  => (RegisterKind::f23,  55),
        "fs2" | "f24"  => (RegisterKind::f24,  56),
                "f25"  => (RegisterKind::f25,  57),
        "fs3" | "f26"  => (RegisterKind::f26,  58),
                "f27"  => (RegisterKind::f27,  59),
        "fs4" | "f28"  => (RegisterKind::f28,  60),
                "f29"  => (RegisterKind::f29,  61),
        "fs5" | "f30"  => (RegisterKind::f30,  62),
                "f31"  => (RegisterKind::f31,  63),
                "fcsr" => (RegisterKind::fcsr, 64),

        _ => return Err(format!("is_register(): invalid register name: {}", word)),
    };

    Ok((register_kind, idx))
}

/// [0-9]?[0-9]* \( `is_register` \)
fn is_memory(word: &str) -> Result<(RegisterKind, usize, u32), String> {
    let errmsg = format!("is_memory(): not memory index: {}", word);
    if Some(')') != word.chars().nth(word.len()-1) {
        return  Err(errmsg);
    }
    let mut add = 0u32;
    let mut s = word.to_string();
    s.pop();  // Delete ')'
    let mut s_chars = s.chars();
    while let Some(c) = s_chars.next() {
        if c.is_ascii_digit() {
            add = add * 10 + (c as u32 - 48);
        } else if c == '(' {
            let mut reg = String::new();
            #[allow(clippy::while_let_on_iterator)]
            while let Some(c) = s_chars.next() {
                reg.push(c);
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
fn is_data_address(word: &str) -> Result<(RegisterKind, usize, String), String> {
    let errmsg = format!("is_data_address(): not data identifier: {}", word);
    if Some(')') != word.chars().nth(word.len()-1) {
        return  Err(errmsg);
    }
    let mut s = word.to_string();
    s.pop();  // Delete ')'
    let mut s_chars = s.chars();

    if let Some(c) = s_chars.next() {
        if let 'A'..='Z' | 'a'..='z' | '_' = c {
            let mut label = c.to_string();

            while let Some(c) = s_chars.next() {
                if let 'A'..='Z' | 'a'..='z' | '_' | '0'..='9' = c {
                    label.push(c);
                } else if c == '(' {
                    let mut reg = String::new();
                    #[allow(clippy::while_let_on_iterator)]
                    while let Some(c) = s_chars.next() {
                        reg.push(c);
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

fn is_hexadecimal(word: &str) -> Option<i32> {
    if word.starts_with("0x") {
        let mut hex: i32 = 0;
        let mut s = word.to_string();
        s.remove(0);
        s.remove(0);  // Delete "0x"

        for h in s.chars() {
            hex = match h {
                '0'..='9' => (hex << 4) + (h as u8 - b'0') as i32,
                'a'..='f' => (hex << 4) + (h as u8 - b'a' + 10) as i32,
                'A'..='F' => (hex << 4) + (h as u8 - b'A' + 10) as i32,
                _ => return None,
            }
        }

        Some(hex)
    } else if word.starts_with("-0x") {
        let mut hex: i32 = 0;
        let mut s = word.to_string();
        s.remove(0);
        s.remove(0);
        s.remove(0);  // Delete "-0x"

        for h in s.chars() {
            hex = match h {
                '0'..='9' => (hex << 4) + (h as u8 - b'0') as i32,
                'a'..='f' => (hex << 4) + (h as u8 - b'a' + 10) as i32,
                'A'..='F' => (hex << 4) + (h as u8 - b'A' + 10) as i32,
                _ => return None,
            }
        }

        Some(-hex)
    } else {
        None
    }
}

fn indicate_word(tokens: &mut Tokens, nol: u32, fi: usize, mut words: std::slice::Iter<&str>) {
    let mut int = 0;
    while let Some(word) = words.next() {
        if 1 < word.len() && word.ends_with(':') {
            let mut word = (*word).to_string();
            let word2 = words.next().unwrap();
            word.pop().unwrap();
            for _ in 0..word2.parse::<usize>().unwrap() {
                int = {
                    if let Some(num) = is_hexadecimal(&word) {
                        num as u32
                    } else if let Ok(num) = word.parse::<i32>() {
                        num as u32
                    } else {
                        word.parse::<u32>().unwrap()
                    }
                };
                tokens.push(TokenKind::INDICATE(IndicateKind::word(int)), nol, fi);
            }
        } else if &word[..] == ":" {
            let word = words.next().unwrap();
            for _ in 1..word.parse::<usize>().unwrap() {
                tokens.push(TokenKind::INDICATE(IndicateKind::word(int)), nol, fi);
            }

        } else {
            int = {
                if let Some(num) = is_hexadecimal(&word) {
                    num as u32
                } else if let Ok(num) = word.parse::<i32>() {
                    num as u32
                } else {
                    word.parse::<u32>().unwrap()
                }
            };
            tokens.push(TokenKind::INDICATE(IndicateKind::word(int)), nol, fi);
        }
    };
}

fn indicate_half(tokens: &mut Tokens, nol: u32, fi: usize, mut words: std::slice::Iter<&str>) {
    let mut half = 0;
    while let Some(word) = words.next() {
        if 1 < word.len() && word.ends_with(':') {
            let mut word = (*word).to_string();
            let word2 = words.next().unwrap();
            word.pop().unwrap();
            for _ in 0..word2.parse::<usize>().unwrap() {
                let half = {
                    if let Some(num) = is_hexadecimal(&word) {
                        num as u16
                    } else if let Ok(num) = word.parse::<i16>() {
                        num as u16
                    } else {
                        word.parse::<u16>().unwrap()
                    }
                };
                tokens.push(TokenKind::INDICATE(IndicateKind::half(half)), nol, fi);
            }
        } else if &word[..] == ":" {
            let word = words.next().unwrap();
            for _ in 1..word.parse::<usize>().unwrap() {
                tokens.push(TokenKind::INDICATE(IndicateKind::half(half)), nol, fi);
            }

        } else {
            half = {
                if let Some(num) = is_hexadecimal(&word) {
                    num as u16
                } else if let Ok(num) = word.parse::<i16>() {
                    num as u16
                } else {
                    word.parse::<u16>().unwrap()
                }
            };
            tokens.push(TokenKind::INDICATE(IndicateKind::half(half)), nol, fi);
        }
    };
}

fn indicate_byte(tokens: &mut Tokens, nol: u32, fi: usize, mut words: std::slice::Iter<&str>) {
    let mut byte = 0;
    while let Some(word) = words.next() {
        if 1 < word.len() && word.ends_with(':') {
            let mut word = (*word).to_string();
            let word2 = words.next().unwrap();
            word.pop().unwrap();
            for _ in 0..word2.parse::<usize>().unwrap() {
                byte = {
                    if let Some(num) = is_hexadecimal(&word) {
                        num as u8
                    } else if let Ok(num) = word.parse::<i8>() {
                        num as u8
                    } else {
                        word.parse::<u8>().unwrap()
                    }
                };
                tokens.push(TokenKind::INDICATE(IndicateKind::byte(byte)), nol, fi);
            }
        } else if &word[..] == ":" {
            let word = words.next().unwrap();
            for _ in 1..word.parse::<usize>().unwrap() {
                tokens.push(TokenKind::INDICATE(IndicateKind::byte(byte)), nol, fi);
            }
        } else {
            byte = {
                if let Some(num) = is_hexadecimal(&word) {
                    num as u8
                } else if let Ok(num) = word.parse::<i8>() {
                    num as u8
                } else {
                    word.parse::<u8>().unwrap()
                }
            };
            tokens.push(TokenKind::INDICATE(IndicateKind::byte(byte)), nol, fi);
        }
    };
}

fn indicate_float(tokens: &mut Tokens, nol: u32, fi: usize, mut words: std::slice::Iter<&str>) {
    let mut float = 0.0;
    while let Some(word) = words.next() {
        if 1 < word.len() && word.ends_with(':') {
            let mut word = (*word).to_string();
            let len = words.next().unwrap().parse::<usize>().unwrap();
            word.pop().unwrap();
            for _ in 0..len {
                float = word.parse::<f32>().unwrap();
                tokens.push(TokenKind::INDICATE(IndicateKind::float(float)), nol, fi);
            }
        } else if &word[..] == ":" {
            let len = words.next().unwrap().parse::<usize>().unwrap();
            for _ in 1..len {
                tokens.push(TokenKind::INDICATE(IndicateKind::float(float)), nol, fi);
            }

        } else {
            float = word.parse::<f32>().unwrap();
            tokens.push(TokenKind::INDICATE(IndicateKind::float(float)), nol, fi);
        }
    };
}

fn indicate_space(word: &str)
    -> u32
{
    let mut word = word.to_string();

    if word.starts_with("0x") {
        let mut hex: u32 = 0;
        word.remove(0);
        word.remove(0);  // Delete "0x"

        for h in word.chars() {
            hex = match h {
                '0'..='9' => (hex << 4) + (h as u8 - b'0') as u32,
                'a'..='f' => (hex << 4) + (h as u8 - b'a' + 10) as u32,
                'A'..='F' => (hex << 4) + (h as u8 - b'A' + 10) as u32,
                _ => panic!("indicate_space(): invalid number: 0x{}", word),
            }
        }

        hex

    } else if let Ok(num) = word.parse::<u32>() {
        num

    } else {
        panic!("indicate_space(): invalid number: {}", word);
    }
}

fn split_words(line: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut line_iter = line.chars();

    while let Some(ch) = line_iter.next() {
        // Skip white space
        if matches!(ch, ' ' | ',' | '\n' | '\r' | '\t') {
            continue;
        }

        // string for .asciiz | literal
        if ch == '"' {
            let mut asciiz = "\"".to_string();
            while let Some(ch2) = line_iter.next() {
                if ch2 != '"' {
                    let mut ch2 = ch2.to_string();
                    if &ch2 == "\\" {
                        let ch3 = line_iter.next().unwrap();
                        ch2 = match ch3 {
                            '\\' => "\\".to_string(),
                            '\'' => "\'".to_string(),
                            '"'  => "\"".to_string(),
                            '0'  => "\0".to_string(),
                            'n'  => "\n".to_string(),
                            'r'  => "\r".to_string(),
                            't'  => "\t".to_string(),
                            'x'  => {
                                if let Some(expect_1) = line_iter.next() {
                                    if expect_1 == '1' {
                                        if let Some(expect_b) = line_iter.next() {
                                            if let 'b'|'B' = expect_b {
                                                "\x1b".to_string()
                                            } else {
                                                format!("\\x1{}", expect_b)
                                            }
                                        } else {
                                            "\\x1".to_string()
                                        }
                                    } else {
                                        format!("\\x{}", expect_1)
                                    }
                                } else {
                                    "\\x".to_string()
                                }
                            },
                            _    => format!("\\{}", ch),
                        };
                    }
                    asciiz.push_str(&ch2);
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

        // ignore comment
        } else if ch == '#' {
            // expect non '\'' or '"' before '#'
            return words;

        // word except string
        } else {
            let mut word = ch.to_string();
            #[allow(clippy::while_let_on_iterator)]
            while let Some(ch2) = line_iter.next() {
                match ch2 {
                    ' ' | ',' | '\n' | '\r' | '\t' => { break; },
                    '#' =>  {  // e.g. $t0#comment
                        words.push(word);
                        return words;
                    },
                    ':' => {  // label
                        word.push(':');
                        break;
                    }
                    _ => {
                        word.push(ch2);
                    },
                }
            }
            words.push(word);
        }
    }

    words
}

