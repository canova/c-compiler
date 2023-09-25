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
            BlockItem::Statement(stmt) => match stmt {
                Statement::Return(_) => {
                    return_indexes.push(idx);
                }
                Statement::Conditional(ref cond) => {
                    if !block_has_return(&cond.if_block)
                        || !cond
                            .else_block
                            .as_ref()
                            .map_or(true, |b| block_has_return(b))
                    {
                        branches_without_return.push(idx);
                    } else {
                        return_indexes.push(idx)
                    }
                }
                Statement::Block(block) => {
                    if !block_has_return(&block.items) {
                        branches_without_return.push(idx);
                    } else {
                        return_indexes.push(idx)
                    }
                }
                Statement::Expression(_) => {}
            },
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
