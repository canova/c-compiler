pub mod asm;
mod helpers;

use self::helpers::*;
use crate::parser::*;
pub use asm::Assembly;

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
            .push(".section __TEXT,__text,regular,pure_instructions");
        self.asm
            .push(".build_version macos, 13, 0 sdk_version 13, 3");

        // main function.
        self.generate_function(program.function)?;
        Ok(())
    }

    fn generate_function(&mut self, func: Function) -> Result<(), String> {
        self.asm.push(format!(".globl _{}", func.name));
        self.asm.push(".p2align 2");
        self.asm.push(format!("_{}:", func.name));
        for stmt in func.body {
            self.generate_statement(stmt)?;
        }
        self.asm.push("ret");
        Ok(())
    }

    fn generate_statement(&mut self, stmt: Statement) -> Result<(), String> {
        match stmt {
            Statement::Return(expr) => match *expr {
                Expr::Constant(Constant::Int(int)) => {
                    self.asm.push(format!("mov w0, #{}", int));
                }
                expression => self.generate_expr(&expression)?,
            },
        }
        Ok(())
    }

    fn generate_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Constant(Constant::Int(int)) => {
                self.asm.push(format!("mov w0, #{}", int));
                Ok(())
            }
            Expr::UnaryOp(unary_op, expr) => {
                self.generate_unary_op(unary_op, expr)?;
                Ok(())
            }
            Expr::BinaryOp(binary_op, lhs, rhs) => {
                self.generate_binary_op(binary_op, lhs, rhs)?;
                Ok(())
            }
            _ => {
                // TODO: Support other types.
                Err(format!("Unexpected expression {:?}", expr))
            }
        }
    }

    fn generate_unary_op(&mut self, unary_op: &UnaryOp, expr: &Box<Expr>) -> Result<(), String> {
        self.generate_expr(expr)?;

        match unary_op {
            UnaryOp::Negation => {
                self.asm.push("neg w0, w0");
            }
            UnaryOp::BitwiseComplement => {
                self.asm.push("mvn w0, w0");
            }
            UnaryOp::LogicalNegation => {
                self.asm.push("cmp w0, #0");
                self.asm.push("mov w0, wzr");
                self.asm.push("cset w0, eq");
            }
        }
        Ok(())
    }

    fn generate_binary_op(
        &mut self,
        binary_op: &BinaryOp,
        lhs: &Box<Expr>,
        rhs: &Box<Expr>,
    ) -> Result<(), String> {
        // FIXME: Currently we do a very naive code generation here by pushing the values
        // to the stack. This is highly inefficient. Since we have a lot of registers
        // available to us in ARM64, we should use them instead.
        self.generate_expr(lhs)?;

        if binary_op.is_short_circuiting_op() {
            self.generate_short_circuiting_op(binary_op, rhs)?;
            return Ok(());
        }

        // It looks like we need to keep 16 byte alignment.
        // https://stackoverflow.com/a/34504752/3582646
        // We first push the value to the stack.
        self.asm.push("stp x0, xzr, [sp, #-16]!");
        self.generate_expr(rhs)?;
        // And then we pop it back to x1.
        self.asm.push("ldp x1, xzr, [sp], #16");

        // lhs is in w1, rhs is in w0.
        match binary_op {
            BinaryOp::Addition => self.asm.push("add w0, w1, w0"),
            BinaryOp::Subtraction => self.asm.push("sub w0, w1, w0"),
            BinaryOp::Multiplication => self.asm.push("mul w0, w1, w0"),
            BinaryOp::Division => {
                // We use signed division here, but we can probably add
                // an optimization with `udiv`.
                self.asm.push("sdiv w0, w1, w0");
            }
            BinaryOp::Equal => {
                self.asm.push("cmp w1, w0");
                self.asm.push("mov w0, wzr");
                self.asm.push("cset w0, eq");
            }
            BinaryOp::NotEqual => {
                self.asm.push("cmp w1, w0");
                self.asm.push("mov w0, wzr");
                self.asm.push("cset w0, ne");
            }
            BinaryOp::LessThan => {
                self.asm.push("cmp w1, w0");
                self.asm.push("mov w0, wzr");
                self.asm.push("cset w0, lt");
            }
            BinaryOp::LessThanOrEq => {
                self.asm.push("cmp w1, w0");
                self.asm.push("mov w0, wzr");
                self.asm.push("cset w0, le");
            }
            BinaryOp::GreaterThan => {
                self.asm.push("cmp w1, w0");
                self.asm.push("mov w0, wzr");
                self.asm.push("cset w0, gt");
            }
            BinaryOp::GreaterThanOrEq => {
                self.asm.push("cmp w1, w0");
                self.asm.push("mov w0, wzr");
                self.asm.push("cset w0, ge");
            }
            BinaryOp::And => {}
            BinaryOp::Or => {}
        }

        Ok(())
    }

    fn generate_short_circuiting_op(
        &mut self,
        binary_op: &BinaryOp,
        rhs: &Box<Expr>,
    ) -> Result<(), String> {
        let end_label = unique_label();

        match binary_op {
            BinaryOp::And => {
                // If lhs is false, we don't need to evaluate rhs.
                self.asm.push("cmp w0, #0");
                self.asm.push("cset w0, ne");
                self.asm.push(format!("cbz w0, {}", end_label));
                self.generate_expr(rhs)?;
                self.asm.push("cmp w0, #0");
                self.asm.push("cset w0, ne");
                self.asm.push(format!("{}:", end_label));
                Ok(())
            }
            BinaryOp::Or => {
                // If lhs is true, we don't need to evaluate rhs.
                self.asm.push("cmp w0, #0");
                self.asm.push("cset w0, ne");
                self.asm.push(format!("cbnz w0, {}", end_label));
                self.generate_expr(rhs)?;
                self.asm.push("cmp w0, #0");
                self.asm.push("cset w0, ne");
                self.asm.push(format!("{}:", end_label));
                Ok(())
            }
            _ => Err(format!("Unexpected binary operator {:?}", binary_op)),
        }
    }
}
