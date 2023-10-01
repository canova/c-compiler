use std::collections::HashMap;

use crate::{
    codegen::{CodegenError, CodegenResult},
    parser::ast::{Block, BlockItem, DeclOrExpr, Expr, Statement, VarDecl, VarSize},
};

#[derive(Debug, PartialEq)]
pub struct CodegenFunction {
    pub stack: FuncStack,
    pub op_stack_depth: usize,
    pub loops: Vec<Loop>,
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
    pub op_count: usize,
}

#[derive(Debug, PartialEq)]
pub struct Loop {
    pub start_label: String,
    pub end_label: String,
}

impl CodegenFunction {
    pub fn new(block: &Block) -> CodegenResult<CodegenFunction> {
        Ok(CodegenFunction {
            stack: block.to_func_stack()?,
            op_stack_depth: 0,
            loops: vec![],
        })
    }
}

impl Block {
    fn to_func_stack(&self) -> CodegenResult<FuncStack> {
        let mut stack = FuncStack {
            var_map: HashMap::new(),
            size: 0,
            op_count: 0,
        };

        self.func_stack(&mut stack)?;

        // Stack size has to be 16 byte aligned.
        // https://stackoverflow.com/a/34504752/3582646
        if stack.size % 16 != 0 {
            stack.size += 16 - (stack.size % 16);
        }

        // Invert all the offsets, it seems to be the common convention.
        for var in stack.var_map.values_mut() {
            match var {
                CodegenVar::StackVar(var) => {
                    var.offset = stack.size - var.offset;
                }
            }
        }

        Ok(stack)
    }

    fn func_stack(&self, stack: &mut FuncStack) -> CodegenResult<()> {
        for item in &self.items {
            match item {
                BlockItem::Declaration(var_decl) => {
                    var_decl.func_stack(stack)?;
                }
                BlockItem::Statement(stmt) => {
                    stmt.func_stack(stack)?;
                }
            }
        }
        Ok(())
    }
}

impl Statement {
    fn func_stack(&self, stack: &mut FuncStack) -> CodegenResult<usize> {
        match self {
            Statement::Expression(expr) | Statement::Return(expr) => {
                // Also look at the arithmetic operations that need to push to stack and
                // and find the largest tree node.
                expr.func_stack(stack)?;
            }
            Statement::Conditional(cond) => {
                // Look at the conditional statements.
                cond.if_stmt.func_stack(stack)?;
                if let Some(else_stmt) = &cond.else_stmt {
                    else_stmt.func_stack(stack)?;
                }
            }
            Statement::Block(block) => {
                block.func_stack(stack)?;
            }
            Statement::While(expr, stmt) => {
                expr.func_stack(stack)?;
                stmt.func_stack(stack)?;
            }
            Statement::DoWhile(stmt, expr) => {
                stmt.func_stack(stack)?;
                expr.func_stack(stack)?;
            }
            Statement::For(for_loop) => {
                for_loop.init.func_stack(stack)?;
                for_loop.condition.func_stack(stack)?;
                for_loop.increment.func_stack(stack)?;
                for_loop.body.func_stack(stack)?;
            }
            Statement::Break | Statement::Continue | Statement::Null => {}
        }

        Ok(0)
    }
}

impl Expr {
    fn func_stack(&self, stack: &mut FuncStack) -> CodegenResult<()> {
        match self {
            Expr::Assignment(_, expr) => expr.func_stack(stack)?,
            Expr::UnaryOp(_, expr) => expr.func_stack(stack)?,
            Expr::BinaryOp(op, lhs, rhs) => {
                lhs.func_stack(stack)?;
                rhs.func_stack(stack)?;
                if !op.is_short_circuiting_op() {
                    // FIXME: Currently we support only word variable size for operations.
                    stack.size += VarSize::Word.to_bytes();
                    stack.var_map.insert(
                        format!("op_{}", stack.op_count),
                        CodegenVar::StackVar(StackVar {
                            size: VarSize::Word,
                            offset: stack.size,
                        }),
                    );
                    stack.op_count += 1;
                }
            }
            Expr::TernaryConditional(ternary) => {
                ternary.condition.func_stack(stack)?;
                ternary.if_expr.func_stack(stack)?;
                ternary.else_expr.func_stack(stack)?;
            }
            Expr::Var(_) => {}
            Expr::Constant(_) => {}
            Expr::Null => {}
        }

        Ok(())
    }
}

impl VarDecl {
    fn func_stack(&self, stack: &mut FuncStack) -> CodegenResult<()> {
        if stack.var_map.contains_key(&self.name) {
            return Err(CodegenError::VarAlreadyDeclared(self.name.clone()));
        }

        let byte_size = self.get_byte_size();
        stack.size += byte_size;
        // We need to invert the offsets at the end.
        stack.var_map.insert(
            self.name.clone(),
            CodegenVar::StackVar(StackVar {
                size: self.size,
                offset: stack.size,
            }),
        );

        Ok(())
    }
}
impl DeclOrExpr {
    fn func_stack(&self, stack: &mut FuncStack) -> CodegenResult<()> {
        match self {
            DeclOrExpr::Expression(expr) => expr.func_stack(stack),
            DeclOrExpr::Declaration(decl) => decl.func_stack(stack),
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
