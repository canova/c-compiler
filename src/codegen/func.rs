use std::collections::HashMap;

use crate::parser::ast::{Expr, Statement, VarSize};

#[derive(Debug, PartialEq)]
pub struct CodegenFunction {
    pub stack: FuncStack,
    pub op_stack_depth: usize,
}

#[derive(Debug, PartialEq)]
pub enum CodegenVar {
    StackVar(StackVar),
}

#[derive(Debug, PartialEq)]
pub struct StackVar {
    /// Size of the variable.
    pub size: VarSize,
    /// Offset from the stack pointer.
    pub offset: usize,
}

#[derive(Debug, PartialEq)]
pub struct FuncStack {
    pub var_map: HashMap<String, CodegenVar>,
    pub size: usize,
}

impl CodegenFunction {
    pub fn new(stmts: &Vec<Statement>) -> Result<CodegenFunction, String> {
        Ok(CodegenFunction {
            stack: CodegenFunction::get_func_stack(stmts)?,
            op_stack_depth: 0,
        })
    }

    pub fn get_func_stack(stmts: &Vec<Statement>) -> Result<FuncStack, String> {
        let mut var_map = HashMap::new();
        let (stack_size, op_stack_count) = Self::get_stack_size(stmts)?;
        let mut stack_offset = stack_size;

        for stmt in stmts {
            #[allow(clippy::single_match)]
            match stmt {
                Statement::VarDecl(var_decl) => {
                    if var_map.contains_key(&var_decl.name) {
                        Err(format!("Variable '{}' is already declared", var_decl.name))?;
                    }

                    let byte_size = var_decl.get_byte_size();
                    stack_offset -= byte_size;
                    var_map.insert(
                        var_decl.name.clone(),
                        CodegenVar::StackVar(StackVar {
                            size: var_decl.size,
                            offset: stack_offset,
                        }),
                    );
                }
                _ => {}
            }
        }

        for i in 0..op_stack_count {
            // FIXME: Currently we only support 4 byte stack variables.
            stack_offset -= VarSize::Word.to_bytes();
            var_map.insert(
                format!("op_{}", i),
                CodegenVar::StackVar(StackVar {
                    size: VarSize::Word,
                    offset: stack_offset,
                }),
            );
        }

        Ok(FuncStack {
            var_map,
            size: stack_size,
        })
    }

    pub fn get_stack_size(stmts: &Vec<Statement>) -> Result<(usize, usize), String> {
        let mut stack_size = 0;
        let mut op_stack_count = 1;

        for stmt in stmts {
            match stmt {
                Statement::VarDecl(var_decl) => {
                    // Look at the variable declarations.
                    stack_size += var_decl.get_byte_size();
                }
                Statement::Expression(expr) | Statement::Return(expr) => {
                    // Also look at the arithmetic operations that need to push to stack and
                    // and find the largest tree node.
                    let expr_stack_size = expr.get_stack_size()?;
                    if expr_stack_size > op_stack_count {
                        op_stack_count = expr_stack_size;
                    }
                }
            }
        }
        // FIXME: Currently we support only word variable size for opeatrions.
        stack_size += op_stack_count * VarSize::Word.to_bytes();

        // Stack size has to be 16 byte aligned.
        // https://stackoverflow.com/a/34504752/3582646
        if stack_size % 16 != 0 {
            stack_size += 16 - (stack_size % 16);
        }

        Ok((stack_size, op_stack_count))
    }
}

impl Expr {
    pub fn get_stack_size(&self) -> Result<usize, String> {
        match self {
            Expr::Assignment(_, expr) => Ok(expr.get_stack_size()?),
            Expr::UnaryOp(_, expr) => Ok(expr.get_stack_size()?),
            Expr::BinaryOp(op, lhs, rhs) => {
                let lhs_size = lhs.get_stack_size()?;
                let rhs_size = rhs.get_stack_size()?;
                let additional_stack = if op.is_short_circuiting_op() { 0 } else { 1 };
                Ok(std::cmp::max(lhs_size, rhs_size) + additional_stack)
            }
            Expr::Var(_) => Ok(0),
            Expr::Constant(_) => Ok(0),
        }
    }
}

impl CodegenVar {
    pub fn get_stack_offset(&self) -> Result<usize, String> {
        match self {
            CodegenVar::StackVar(var) => Ok(var.offset),
        }
    }
}
