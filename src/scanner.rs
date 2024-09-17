use crate::LoxErr;

pub struct Scanner {
    contents: String,
}

impl Scanner {
    pub fn new(contents: &str) -> Self {
        Self {
            contents: contents.to_string(),
        }
    }

    pub fn scan_tokens(&self) -> Result<Vec<Token>, LoxErr> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Token {}
