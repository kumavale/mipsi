use super::super::token::*;

impl Tokens {
    pub fn new() -> Self {
        let token: Vec<Token> = Vec::new();
        Tokens { token,
                 data_area_now: true,
                 idx: 0,
                 foremost: true,
                 length: 0,
                 addresses: Vec::new(),
                 filenames: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        self.token.clear();
        self.idx = 0;
        self.foremost = true;
        self.length = 0;
        self.data_area_now = true;
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, kind: TokenKind, line: u32, filename_idx: usize) {
        self.length += 1;
        self.token.push(Token { kind, line, filename_idx });
    }

    pub fn pop(&mut self) -> Option<Token> {
        if 0 < self.length {
            self.length -= 1;
            if self.length == 0 {
                self.idx = 0;
                self.foremost = true;
            }
            if self.idx > self.length {
                self.idx = self.length - 1;
            }
            self.token.pop()
        } else {
            None
        }
    }

    pub fn back_idx(&mut self) {
        if 0 < self.idx {
            self.idx -= 1;
        }
        if self.idx == 0 {
            self.foremost = true;
        }
    }

    pub fn add_address(&mut self, label: String, token_index: usize) {
        self.addresses.push((label, token_index));
    }

    pub fn add_file(&mut self, file: &str) {
        self.filenames.push(file.to_string());
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.foremost = true;
        self.idx = 0;
    }

    pub fn consume(&mut self) -> Option<Token> {
        if self.foremost {
            self.foremost = false;

            // `TOKEN_TRACE=1 cargo run`
            if std::env::var("TOKEN_TRACE").is_ok() {
                println!("line:index, kind");
                println!("{}:{:?}:{:?},\t{:?}",
                    &self.filenames[self.token[0].filename_idx],
                    &self.token[0].line, &self.idx, &self.token[0].kind);
            }

            Some(self.token[0].clone())
        } else if self.idx+1 < self.length {
            self.idx += 1;

            // `TOKEN_TRACE=1 cargo run`
            if std::env::var("TOKEN_TRACE").is_ok() {
                println!("{}:{:?}:{:?},\t{:?}",
                    &self.filenames[self.token[self.idx].filename_idx],
                    &self.token[self.idx].line, &self.idx,  &self.token[self.idx].kind);
            }

            Some(self.token[self.idx].clone())
        } else {
            // `TOKEN_TRACE=1 cargo run`
            if std::env::var("TOKEN_TRACE").is_ok() {
                println!("EOF");
            }

            None
        }
    }

    pub fn goto(&mut self, idx: usize) {
        // `TOKEN_TRACE=1 cargo run`
        if std::env::var("TOKEN_TRACE").is_ok() {
            println!(" |\n | GOTO: {}:{:?}:{:?},\t{:?}\n |",
                &self.filenames[self.token[idx+1].filename_idx],
                &self.token[idx+1].line, idx+1,  &self.token[idx+1].kind);
        }

        if idx == 0 {
            self.foremost = true;
        }
        self.idx = idx;
    }

    pub fn idx(&self) -> usize {
        self.idx
    }

    pub fn kind(&mut self) -> &mut TokenKind {
        &mut self.token[self.idx].kind
    }

    pub fn next(&self) -> Option<Token> {
        if self.idx + 1 < self.length {
            Some(self.token[self.idx+1].clone())
        } else {
            None
        }
    }

    pub fn is_none(&self) -> bool {
        self.idx + 1 >= self.length
    }

    /// Get data index of String same as TokenKind::ADDRESS() from TokenKind::LABEL()
    pub fn expect_address(&self) -> Result<usize, String> {
        if let TokenKind::ADDRESS(s) = &self.token[self.idx].kind {
            let line = self.token[self.idx].line;
            for t in &self.token {
                if let TokenKind::LABEL(name, _, idx) = &t.kind {
                    if *s == *name {
                        return Ok((*idx)
                            .ok_or_else(|| format!("{}: invalid address: {}", line, s))?);
                    }
                }
            }
            Err(format!("{}: invalid address: {}", line, s))
        } else {
            let t = &self.token[self.idx];
            Err(format!("{}: expect TokenKind::ADDRESS(String). but got: {:?}", t.line, t.kind))
        }
    }

    /// Get label index of String same as TokenKind::ADDRESS() from TokenKind::LABEL()
    pub fn expect_label(&self) -> Result<usize, String> {
        if let TokenKind::ADDRESS(s) = &self.token[self.idx].kind {
            for a in &self.addresses {
                if *s == *a.0 {
                    return Ok(a.1);
                }
            }
            let line = self.token[self.idx].line;
            Err(format!("{}: invalid address: {}", line, s))
        } else {
            let t = &self.token[self.idx];
            Err(format!("{}: expect TokenKind::ADDRESS(String). but got: {:?}", t.line, t.kind))
        }
    }

    pub fn expect_instruction(&self) -> Result<InstructionKind, String> {
        if let TokenKind::INSTRUCTION(k) = self.token[self.idx].kind {
            Ok(k)
        } else {
            let t = &self.token[self.idx];
            Err(format!("{}: expect TokenKind::INSTRUCTION(InstructionKind). but got: {:?}", t.line, t.kind))
        }
    }

    pub fn expect_register(&self) -> Result<usize, String> {
        if let TokenKind::REGISTER(_, i) = self.token[self.idx].kind {
            Ok(i)
        } else {
            let t = &self.token[self.idx];
            Err(format!("{}: expect TokenKind::REGISTER(RegisterKind, usize). but got: {:?}", t.line, t.kind))
        }
    }

    /// Return: Ok((register_idx, append idx))
    pub fn expect_memory(&self) -> Result<(usize, i32), String> {
        if let TokenKind::MEMORY(_, i, j) = self.token[self.idx].kind {
            Ok((i, j))
        } else {
            let t = &self.token[self.idx];
            Err(format!("{}: expect TokenKind::MEMORY(RegisterKind, usize, i32). but got: {:?}", t.line, t.kind))
        }
    }

    /// Return: Ok((register_idx, data index))
    pub fn expect_data(&self) -> Result<(usize, usize), String> {
        if let TokenKind::DATA(_, r_i, s) = &self.token[self.idx].kind {
            for t in &self.token {
                if let TokenKind::LABEL(name, _, Some(d_i)) = &t.kind {
                    if *s == *name {
                        return Ok((*r_i, *d_i));
                    }
                }
            }
        }
        let t = &self.token[self.idx];
        Err(format!("{}: expect TokenKind::DATA(RegisterKind, usize, String). but got: {:?}", t.line, t.kind))
    }

    pub fn expect_integer(&self) -> Result<i32, String> {
        if let TokenKind::INTEGER(i) = self.token[self.idx].kind {
            Ok(i)
        } else {
            let t = &self.token[self.idx];
            Err(format!("{}: expect TokenKind::INTEGER(i32). but got: {:?}", t.line, t.kind))
        }
    }

    pub fn expect_literal(&self) -> Result<String, String> {
        if let TokenKind::LITERAL(l) = &self.token[self.idx].kind {
            Ok(l.to_string())
        } else {
            let t = &self.token[self.idx];
            Err(format!("{}: expect TokenKind::LITERAL(String). but got: {:?}", t.line, t.kind))
        }
    }

    pub fn expect_eol(&self) -> Result<(), String> {
        if let TokenKind::EOL = self.token[self.idx].kind {
            Ok(())
        } else {
            let t = &self.token[self.idx];
            Err(format!("{}: expect TokenKind::EOL. but got: {:?}", t.line, t.kind))
        }
    }
}

