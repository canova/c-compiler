pub mod asm;
mod func;
mod helpers;

use self::helpers::*;
pub use self::{asm::Assembly, func::*};
use crate::parser::*;

#[derive(Debug, PartialEq)]
pub struct ARMCodegen {
    asm: Assembly,
    funcs: Vec<CodegenFunction>,
}

impl ARMCodegen {
    pub fn new() -> ARMCodegen {
        ARMCodegen {
            asm: Assembly::new(),
            funcs: Vec::new(),
        }
    }

    pub fn generate(mut self, program: Program) -> Result<String, String> {
        self.generate_program(program)?;
        Ok(self.asm.to_string())
    }

    fn get_current_func(&self) -> Result<&CodegenFunction, String> {
        self.funcs.last().ok_or("No function found".to_string())
    }

    fn get_current_func_mut(&mut self) -> Result<&mut CodegenFunction, String> {
        self.funcs.last_mut().ok_or("No function found".to_string())
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

        self.funcs.push(CodegenFunction::new(&func.body)?);

        // Push the stack in the function prologue.
        self.asm.push(format!(
            "sub sp, sp, #{}",
            self.funcs.last().unwrap().stack.size
        ));
        for stmt in &func.body {
            self.generate_statement(stmt)?;
        }
        // Pop the stack in the function epilogue.
        self.asm.push(format!(
            "add sp, sp, #{}",
            self.funcs.last().unwrap().stack.size
        ));

        if self.funcs.len() == 1 {
            // If there is only one function, that means that it's the main function.
            // TODO: This is not the best way of checking if the main has no return.
            // We should improve this.
            let function_has_return = func
                .body
                .iter()
                .any(|stmt| matches!(stmt, Statement::Return(_)));
            if !function_has_return {
                // If the main function doesn't have a return statement, we need to
                // return 0 as per the C standard. But that's not the case for the other
                // functions.
                self.asm.push("mov w0, #0");
            }
        }

        self.funcs.pop();
        self.asm.push("ret");
        Ok(())
    }

    fn generate_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Return(expr) => match expr.as_ref() {
                Expr::Constant(Constant::Int(int)) => {
                    self.asm.push(format!("mov w0, #{}", int));
                }
                expression => self.generate_expr(&expression)?,
            },
            Statement::VarDecl(var_decl) => {
                if let Some(expr) = &var_decl.initializer {
                    self.generate_expr(&expr)?;
                } else {
                    self.asm.push("mov w0, #0");
                }

                let codegen_var = self
                    .get_current_func()?
                    .stack
                    .var_map
                    .get(&var_decl.name)
                    .ok_or(format!("Variable '{}' not found", var_decl.name))?;

                match codegen_var {
                    CodegenVar::StackVar(stack_var) => {
                        self.asm
                            .push(format!("str w0, [sp, #{}]", stack_var.offset));
                    }
                }
            }
            Statement::Expression(expr) => {
                self.generate_expr(&expr)?;
            }
        }
        Ok(())
    }

    fn generate_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Constant(Constant::Int(int)) => {
                self.asm.push(format!("mov w0, #{}", int));
                Ok(())
            }
            Expr::Constant(_) => {
                // TODO: Support the other types later.
                todo!("Only integer constants are supported")
            }
            Expr::UnaryOp(unary_op, expr) => {
                self.generate_unary_op(unary_op, expr)?;
                Ok(())
            }
            Expr::BinaryOp(binary_op, lhs, rhs) => {
                self.generate_binary_op(binary_op, lhs, rhs)?;
                Ok(())
            }
            Expr::Var(var_name) => {
                let codegen_var = self
                    .get_current_func()?
                    .stack
                    .var_map
                    .get(var_name)
                    .ok_or(format!("Variable '{}' not found", var_name))?;

                match codegen_var {
                    CodegenVar::StackVar(stack_var) => {
                        self.asm
                            .push(format!("ldr w0, [sp, #{}]", stack_var.offset));
                    }
                }
                Ok(())
            }
            Expr::Assignment(name, expr) => {
                self.generate_expr(expr)?;

                let codegen_var = self
                    .get_current_func()?
                    .stack
                    .var_map
                    .get(name)
                    .ok_or(format!("Variable '{}' not found", name))?;

                match codegen_var {
                    CodegenVar::StackVar(stack_var) => {
                        self.asm
                            .push(format!("str w0, [sp, #{}]", stack_var.offset));
                    }
                }
                Ok(())
            }
        }
    }

    fn generate_unary_op(&mut self, unary_op: &UnaryOp, expr: &Expr) -> Result<(), String> {
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
        lhs: &Expr,
        rhs: &Expr,
    ) -> Result<(), String> {
        self.generate_expr(lhs)?;

        if binary_op.is_short_circuiting_op() {
            self.generate_short_circuiting_op(binary_op, rhs)?;
            return Ok(());
        }

        let func = self.get_current_func_mut()?;
        let op_var = func
            .stack
            .var_map
            .get(&format!("op_{}", func.op_stack_depth))
            .unwrap();
        func.op_stack_depth += 1;
        let stack_offset = op_var.get_stack_offset()?;

        // We first push the value to the stack.
        self.asm.push(format!("str w0, [sp, #{}]", stack_offset));
        self.generate_expr(rhs)?;
        // And then we pop it back to w1.
        self.asm.push(format!("ldr w1, [sp, #{}]", stack_offset));
        self.get_current_func_mut()?.op_stack_depth -= 1;

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
            BinaryOp::Modulo => {
                self.asm.push("sdiv w2, w1, w0");
                self.asm.push("msub w0, w2, w0, w1");
            }
            BinaryOp::BitwiseAnd => {
                self.asm.push("and w0, w1, w0");
            }
            BinaryOp::BitwiseOr => {
                self.asm.push("orr w0, w1, w0");
            }
            BinaryOp::BitwiseXor => {
                self.asm.push("eor w0, w1, w0");
            }
            BinaryOp::BitwiseShiftLeft => {
                self.asm.push("lsl w0, w1, w0");
            }
            BinaryOp::BitwiseShiftRight => {
                self.asm.push("lsr w0, w1, w0");
            }
            // These are short circuiting operators, so we don't need to do anything here.
            BinaryOp::And => {}
            BinaryOp::Or => {}
        }

        Ok(())
    }

    fn generate_short_circuiting_op(
        &mut self,
        binary_op: &BinaryOp,
        rhs: &Expr,
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
