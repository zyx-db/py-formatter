use std::collections::HashMap;

use crate::utils::{
    clean_expression, fix_function_def, group_expressions, is_block_start, leading_spaces,
};

pub struct Formatter {
    pub lines: Vec<String>,
    pub block_ends: HashMap<usize, usize>,
}

impl Formatter {
    pub fn new(mut lines: Vec<String>) -> Self {
        lines.push("".to_owned());
        Self {
            lines,
            block_ends: HashMap::new(),
        }
    }

    pub fn compute_blocks(&mut self) {
        let mut stack = Vec::new();

        for (i, line) in self.lines.iter().enumerate() {
            let curr_depth = leading_spaces(line);
            while let Some((depth, idx)) = stack.last() {
                if *depth >= curr_depth {
                    self.block_ends.insert(*idx, i);
                    stack.pop();
                } else {
                    break;
                }
            }

            if is_block_start(line) {
                stack.push((curr_depth, i));
            }
        }

        self.lines = self
            .lines
            .iter()
            .map(|line| line.trim().to_owned())
            .collect();
    }

    pub fn one_line_function(&self) -> String {
        format!(
            "{}\n\treturn {}",
            fix_function_def(self.lines[0].clone()),
            self.compress(1, self.lines.len())
        )
    }

    pub fn compress(&self, start: usize, end: usize) -> String {
        let mut expressions = Vec::new();
        let mut expression;

        let mut i = start;
        while i < end {
            if self.lines[i].len() == 0 {
                break;
            }
            if self.lines[i].starts_with("if") {
                (i, expression) = self.compress_condition(i);
                expressions.push(expression);
            } else {
                expressions.push(clean_expression(self.lines[i].clone()));
                i += 1;
            }
        }

        group_expressions(expressions)
    }
}
