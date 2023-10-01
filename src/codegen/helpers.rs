use crate::parser::ast::{BlockItem, Statement};

use std::{
    sync::atomic::{AtomicUsize, Ordering},
    vec,
};

static LABEL_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn unique_label() -> String {
    format!("L{}", LABEL_COUNTER.fetch_add(1, Ordering::SeqCst))
}

// TODO: Create a new struct for Block and move this to its method.
// FIXME: pass a block instead.
pub fn block_has_return(block_items: &[BlockItem]) -> bool {
    let mut return_indexes = vec![];
    let mut branches_without_return = vec![];

    for (idx, item) in block_items.iter().enumerate() {
        match item {
            BlockItem::Declaration(_) => {}
            BlockItem::Statement(stmt) => {
                if let Some(has_return) = stmt.has_return() {
                    if has_return {
                        return_indexes.push(idx);
                    } else {
                        branches_without_return.push(idx);
                    }
                }
            }
        }
    }

    // Check if there are branches without return that don't lead to a return.
    for idx in branches_without_return {
        if return_indexes.iter().all(|&i| i < idx) {
            println!("Warning: Not all branches lead to a return. This is undefined behavior!");
        }
    }

    !return_indexes.is_empty()
}

impl Statement {
    // The return type is not ideal but it does the job.
    // Returning `None` means that the statement does not have a return.
    // `Some(false)` means that the statement has has a branch without return.
    // `Some(true)` means that it has a return.
    pub fn has_return(&self) -> Option<bool> {
        match self {
            Statement::Return(_) => Some(true),
            Statement::Conditional(ref cond) => {
                let if_has_return = cond.if_stmt.has_return().unwrap_or(false);
                let else_has_return = cond
                    .else_stmt
                    .as_ref()
                    .map_or(Some(true), |b| b.has_return())
                    .unwrap_or(false);

                Some(if_has_return && else_has_return)
            }
            Statement::Block(block) => Some(block_has_return(&block.items)),
            Statement::Expression(_) => None,
            Statement::While(_, stmt) => stmt.has_return(),
            Statement::DoWhile(stmt, _) => stmt.has_return(),
            Statement::For(for_loop) => for_loop.body.has_return(),
            Statement::Break | Statement::Continue | Statement::Null => None,
        }
    }
}
