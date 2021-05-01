mod expand;
mod export;
mod new;
mod update;

use super::*;
use node::{Node, Value};
use std::{borrow::Borrow, error::Error, fmt, ops};
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
    tree_table: Vec<Vec<Box<dyn Node>>>,
    refs_table: Vec<Vec<Vec<(usize, usize)>>>,
    calculated_table: Vec<Vec<Value>>,
    current_pos: usize,
}

impl Table {
    fn build_tree(raw_string: String) -> (Box<dyn Node>, Vec<(usize, usize)>) {
        if let Ok(primitive_token_string) = token::primitive_parse(raw_string) {
            if let Ok(token_string) = token::parse(primitive_token_string) {
                return if token_string.len() == 0 {
                    (Box::new(Value::Empty) as Box<dyn Node>, vec![])
                } else {
                    node::parse(&token_string)
                };
            }
        }
        (Box::new(Value::Error) as Box<dyn Node>, vec![])
    }

    fn calc(
        x: usize,
        y: usize,
        tree_table: &Vec<Vec<Box<dyn Node>>>,
        refs_table: &Vec<Vec<Vec<(usize, usize)>>>,
        calculated_table: &mut Vec<Vec<Value>>,
    ) {
        calculated_table[y][x] = tree_table[y][x].calc(&calculated_table);
        for (x, y) in &refs_table[y][x] {
            Self::calc(*x, *y, tree_table, refs_table, calculated_table);
        }
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
        if self.current_pos < self.calculated_table.len() {
            let item = self.calculated_table[self.current_pos].clone();
            self.current_pos += 1;
            Some(item)
        } else {
            self.current_pos = 0;
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let num_of_remaining_items = self.calculated_table.len() - self.current_pos;
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
            dump = format!("{}\n", dump);
        }
        write!(f, "{}", dump)
    }
}