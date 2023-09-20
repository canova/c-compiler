#[derive(Debug, PartialEq)]
pub struct Assembly {
    asm: Vec<String>,
}

impl Assembly {
    pub fn new() -> Assembly {
        Assembly { asm: Vec::new() }
    }

    pub fn push<S: Into<String>>(&mut self, string: S) {
        self.asm.push(string.into())
    }
}

impl ToString for Assembly {
    fn to_string(&self) -> String {
        self.asm.join("\n")
    }
}
