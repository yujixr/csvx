mod calc;
mod export;
mod insert;
mod new;
mod remove;
mod update;

use super::*;
use node::{ThreadSafeNode, Value};
use std::{borrow::Borrow, collections::HashMap, error::Error, fmt, ops};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TableError {
    #[error("Unknown error occurred while parsing CSV")]
    CSVParseError,
    #[error("the position {x},{y} is out of range")]
    OutOfRange { x: usize, y: usize },
}

/// CSVX table
pub struct Table {
    raw_table: Vec<Vec<String>>,
    tree_table: Vec<Vec<Box<ThreadSafeNode>>>,
    ref_src_table: Vec<Vec<Vec<(usize, usize)>>>,
    dependents_table: Vec<Vec<HashMap<(usize, usize), u32>>>,
    calculated_table: Vec<Vec<Value>>,
    current_pos_y: usize,
}

impl Table {
    /// Retrieve internal raw table.
    pub fn get_raw_table(&self) -> &Vec<Vec<String>> {
        &self.raw_table
    }

    /// Retrieve internal calculated table.
    pub fn get_calculated_table(&self) -> &Vec<Vec<Value>> {
        &self.calculated_table
    }

    fn build_tree<T: Borrow<str>>(raw_string: &T) -> (Box<ThreadSafeNode>, Vec<(usize, usize)>) {
        if let Ok(primitive_token_string) = token::primitive_parse(raw_string) {
            if let Ok(token_string) = token::parse(primitive_token_string) {
                return if token_string.len() == 0 {
                    (Box::new(Value::Empty) as Box<ThreadSafeNode>, vec![])
                } else {
                    node::parse(&token_string)
                };
            }
        }
        (Box::new(Value::Error) as Box<ThreadSafeNode>, vec![])
    }
}

impl ops::Index<usize> for Table {
    type Output = Vec<Value>;
    fn index(&self, y: usize) -> &Self::Output {
        &self.calculated_table[y]
    }
}

impl Iterator for Table {
    type Item = Vec<Value>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos_y < self.calculated_table.len() {
            let item = self.calculated_table[self.current_pos_y].clone();
            self.current_pos_y += 1;
            Some(item)
        } else {
            self.current_pos_y = 0;
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let num_of_remaining_items = self.calculated_table.len() - self.current_pos_y;
        (num_of_remaining_items, Some(num_of_remaining_items))
    }
}

impl ExactSizeIterator for Table {}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dump = String::new();
        for y in 0..self.calculated_table.len() {
            for x in 0..self.calculated_table[y].len() {
                dump = format!("{}{}\t", dump, self.calculated_table[y][x]);
            }
            dump.push('\n');
        }
        write!(f, "{}", dump)
    }
}
