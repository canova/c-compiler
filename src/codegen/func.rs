use std::collections::HashMap;

use crate::{
    codegen::{CodegenError, CodegenResult},
    parser::ast::{Block, BlockItem, Expr, Statement, VarSize},
};

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
    pub fn new(block: &Block) -> CodegenResult<CodegenFunction> {
        Ok(CodegenFunction {
            stack: block.get_stack()?,
            op_stack_depth: 0,
        })
    }
}

impl Block {
    pub fn get_stack(&self) -> CodegenResult<FuncStack> {
        let mut var_map = HashMap::new();
        let (stack_size, op_stack_count) = self.get_stack_size()?;
        let mut stack_offset = stack_size;

        for item in &self.items {
            match item {
                BlockItem::Declaration(var_decl) => {
                    if var_map.contains_key(&var_decl.name) {
                        return Err(CodegenError::VarAlreadyDeclared(var_decl.name.clone()));
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
                BlockItem::Statement(_) => {}
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

    pub fn get_stack_size(&self) -> CodegenResult<(usize, usize)> {
        let mut stack_size = 0;
        let mut op_stack_count = 0;
        let mut max_branch_stack_size = 0;

        for item in &self.items {
            match item {
                BlockItem::Declaration(var_decl) => {
                    // Look at the variable declarations.
                    stack_size += var_decl.get_byte_size();
                }
                BlockItem::Statement(stmt) => {
                    // Look at the statements.
                    let (stmt_stack_size, stmt_op_stack_count) = stmt.get_stack_size()?;
                    max_branch_stack_size = std::cmp::max(stmt_stack_size, max_branch_stack_size);
                    op_stack_count = std::cmp::max(stmt_op_stack_count, op_stack_count);
                }
            }
        }
        // FIXME: Currently we support only word variable size for operations.
        stack_size += max_branch_stack_size + op_stack_count * VarSize::Word.to_bytes();

        // Stack size has to be 16 byte aligned.
        // https://stackoverflow.com/a/34504752/3582646
        if stack_size % 16 != 0 {
            stack_size += 16 - (stack_size % 16);
        }

        Ok((stack_size, op_stack_count))
    }
}

impl Statement {
    pub fn get_stack_size(&self) -> CodegenResult<(usize, usize)> {
        let mut op_stack_count = 0;
        let mut max_branch_stack_size = 0;

        match self {
            Statement::Expression(expr) | Statement::Return(expr) => {
                // Also look at the arithmetic operations that need to push to stack and
                // and find the largest tree node.
                let expr_stack_size = expr.get_stack_size()?;
                if expr_stack_size > op_stack_count {
                    op_stack_count = expr_stack_size;
                }
            }
            Statement::Conditional(cond) => {
                // Look at the conditional statements.
                let (if_stack_size, if_op_stack_count) = cond.if_stmt.get_stack_size()?;
                let (else_stack_size, else_op_stack_count) =
                    if let Some(else_stmt) = &cond.else_stmt {
                        else_stmt.get_stack_size()?
                    } else {
                        (0, 0)
                    };

                max_branch_stack_size = std::cmp::max(
                    std::cmp::max(if_stack_size, else_stack_size),
                    max_branch_stack_size,
                );
                op_stack_count = std::cmp::max(
                    std::cmp::max(if_op_stack_count, else_op_stack_count),
                    op_stack_count,
                );
            }
            Statement::Block(block) => {
                let (block_stack_size, block_op_stack_count) = block.get_stack_size()?;
                max_branch_stack_size = std::cmp::max(block_stack_size, max_branch_stack_size);
                op_stack_count = std::cmp::max(block_op_stack_count, block_stack_size);
            }
        }

        // FIXME: Currently we support only word variable size for operations.
        let stack_size = max_branch_stack_size + op_stack_count * VarSize::Word.to_bytes();

        Ok((stack_size, op_stack_count))
    }
}

impl Expr {
    pub fn get_stack_size(&self) -> CodegenResult<usize> {
        match self {
            Expr::Assignment(_, expr) => Ok(expr.get_stack_size()?),
            Expr::UnaryOp(_, expr) => Ok(expr.get_stack_size()?),
            Expr::BinaryOp(op, lhs, rhs) => {
                let lhs_size = lhs.get_stack_size()?;
                let rhs_size = rhs.get_stack_size()?;
                let additional_stack = if op.is_short_circuiting_op() { 0 } else { 1 };
                Ok(std::cmp::max(lhs_size, rhs_size) + additional_stack)
            }
            Expr::TernaryConditional(ternary) => {
                let cond_size = ternary.condition.get_stack_size()?;
                let if_size = ternary.if_expr.get_stack_size()?;
                let else_size = ternary.else_expr.get_stack_size()?;
                Ok(std::cmp::max(std::cmp::max(cond_size, if_size), else_size))
            }
            Expr::Var(_) => Ok(0),
            Expr::Constant(_) => Ok(0),
        }
    }
}

impl CodegenVar {
    pub fn get_stack_offset(&self) -> CodegenResult<usize> {
        match self {
            CodegenVar::StackVar(var) => Ok(var.offset),
        }
    }
}
