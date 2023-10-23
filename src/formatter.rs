use std::collections::{HashMap, HashSet};

use crate::utils::{
    fix_assignment, fix_function_def, group_expressions, is_block_start, leading_spaces,
};

pub struct Formatter {
    pub lines: Vec<String>,
    pub block_ends: HashMap<usize, usize>,
    pub loop_depths: Vec<usize>,
}

impl Formatter {
    pub fn new(mut lines: Vec<String>) -> Self {
        lines.push("".to_owned());
        let n = lines.len();
        Self {
            lines,
            block_ends: HashMap::new(),
            loop_depths: vec![0; n],
        }
    }

    pub fn compute_blocks_and_depth(&mut self) {
        let mut stack = Vec::new();
        let mut loop_depth = 0;
        let mut loop_starts = HashSet::new();

        for (i, line) in self.lines.iter().enumerate() {
            if line.trim().starts_with("for") || line.trim().starts_with("while") {
                loop_depth += 1;
                loop_starts.insert(i);
            }
            self.loop_depths[i] = loop_depth;

            let tab_depth = leading_spaces(line);
            while let Some((depth, idx)) = stack.last() {
                if *depth >= tab_depth {
                    self.block_ends.insert(*idx, i);
                    if loop_starts.contains(idx) {
                        loop_depth -= 1;
                    }
                    stack.pop();
                } else {
                    break;
                }
            }

            if is_block_start(line) {
                stack.push((tab_depth, i));
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
            "{}\n\treturn {}[1]",
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
            } else if self.lines[i].starts_with("for") {
                let mut for_expression = self.lines[i].clone();
                for_expression.pop();
                let block_end = self.block_ends[&i];
                let for_group = format!(
                    "group({},({} {}))",
                    self.loop_depths[i],
                    self.compress(i + 1, block_end),
                    for_expression
                );
                expressions.push(for_group);
                i = block_end;
            } else {
                expressions.push(self.clean_expression(i));
                i += 1;
            }
        }

        group_expressions(self.loop_depths[start], expressions)
    }

    fn clean_expression(&self, i: usize) -> String {
        let e = self.lines[i].clone();
        if e.starts_with("return") {
            format!("((),{})", e[6..].trim())
        } else if e == "break" {
            format!("(...,{})", self.loop_depths[i])
        } else {
            fix_assignment(e)
        }
    }
}
