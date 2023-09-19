use crate::parser::*;

#[derive(Debug, PartialEq)]
pub struct Assembly {
    asm: Vec<String>,
}

impl Assembly {
    fn new() -> Assembly {
        Assembly { asm: Vec::new() }
    }

    fn push<S: Into<String>>(&mut self, string: S) {
        self.asm.push(string.into())
    }
}

impl ToString for Assembly {
    fn to_string(&self) -> String {
        self.asm.join("\n")
    }
}

#[derive(Debug, PartialEq)]
pub struct ARMCodegen {
    asm: Assembly,
}

impl ARMCodegen {
    pub fn new() -> ARMCodegen {
        ARMCodegen {
            asm: Assembly::new(),
        }
    }

    pub fn generate(mut self, program: Program) -> Result<String, String> {
        self.generate_program(program)?;
        Ok(self.asm.to_string())
    }

    fn generate_program(&mut self, program: Program) -> Result<(), String> {
        // Header.
        self.asm
            .push(".section	__TEXT,__text,regular,pure_instructions");
        self.asm
            .push(".build_version macos, 13, 0	sdk_version 13, 3");
        self.asm.push(".globl	_main");
        self.asm.push(".p2align	2");

        // main function.

        self.generate_function(program.function)?;
        Ok(())
    }

    fn generate_function(&mut self, func: Function) -> Result<(), String> {
        self.asm.push(format!("_{}:", func.name));
        for stmt in func.body {
            self.generate_statement(stmt)?;
        }
        self.asm.push("ret");
        Ok(())
    }

    fn generate_statement(&mut self, stmt: Statement) -> Result<(), String> {
        match stmt {
            Statement::Return(expr) => {
                match *expr {
                    Expr::Int(int) => {
                        self.asm.push(format!("mov	w0, #{}", int));
                    }
                    _ => {
                        // TODO: Support other types.
                        return Err(format!(
                            "Only int return expression is supported but got {:?}",
                            expr
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}
