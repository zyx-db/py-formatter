use std::collections::HashMap;

pub struct Formatter {
    lines: Vec<String>,
    block_ends: HashMap<usize, usize>,
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

    fn compress(&self, start: usize, end: usize) -> String {
        let mut expressions = Vec::new();

        let mut i = start;
        while i < end {
            if self.lines[i].len() == 0 {
                break;
            }
            if self.lines[i].starts_with("if") {
                let mut if_clauses = Vec::new();
                let mut else_clause = "None".to_owned();

                loop {
                    let block_end = self.block_ends[&i];
                    let line_len = self.lines[i].len();
                    let boolean_expression = {
                        if self.lines[i].starts_with("if") {
                            &self.lines[i][2..line_len - 1]
                        } else {
                            &self.lines[i][4..line_len - 1]
                        }
                    }
                    .trim()
                    .to_owned();

                    if_clauses.push((boolean_expression, self.compress(i + 1, block_end)));
                    i = block_end;

                    if !self.lines[i].starts_with("if") && !self.lines[i].starts_with("elif") {
                        break;
                    }
                }

                if self.lines[i].starts_with("else") {
                    let block_end = self.block_ends[&i];
                    else_clause = self.compress(i + 1, block_end);
                    i = block_end;
                }

                expressions.push(if_clauses.iter().rev().fold(
                    else_clause,
                    |prev, (condition, block)| {
                        format!("[lambda:{},lambda:{}][{}]()", prev, block, condition)
                    },
                ))
            } else {
                expressions.push(clean_expression(self.lines[i].clone()));
                i += 1;
            }
        }

        group_expressions(expressions)
    }
}

fn is_block_start(line: &str) -> bool {
    ["if", "elif", "else", "for", "while", "def"]
        .into_iter()
        .any(|keyword| line.trim().starts_with(keyword))
}

fn leading_spaces(s: &String) -> usize {
    s.chars()
        .take_while(|ch| ch.is_whitespace() && *ch != '\n')
        .count()
}

fn fix_function_def(function_def: String) -> String {
    let g_def = "g=lambda x:next(filter(lambda x:x is not None,x),None)";
    let n_def = "n=lambda x:None";

    let closing_idx = function_def.rfind(')').unwrap();

    if function_def.chars().nth(closing_idx - 1).unwrap() == '(' {
        format!(
            "{}{},{}{}",
            &function_def[..closing_idx],
            g_def,
            n_def,
            &function_def[closing_idx..]
        )
    } else {
        format!(
            "{},{},{}{}",
            &function_def[..closing_idx],
            g_def,
            n_def,
            &function_def[closing_idx..]
        )
    }
}

pub fn group_expressions(expressions: Vec<String>) -> String {
    if expressions.len() == 1 {
        return expressions[0].clone();
    }
    let expression_list = expressions.join(",");

    format!("g([{}])", expression_list)
}

fn clean_expression(e: String) -> String {
    if e.starts_with("return") {
        e[6..].trim().to_owned()
    } else {
        format!("n({})", fix_assignment(e.to_owned()))
    }
}

fn fix_assignment(e: String) -> String {
    if let Some((mut before_eq, mut after_eq)) = e.split_once('=') {
        if before_eq.chars().last().unwrap() == ':' || after_eq.chars().next().unwrap() == '=' {
            return e;
        }

        before_eq = before_eq.trim();
        after_eq = after_eq.trim();

        if let (Some(bracket_start), Some(bracket_end)) =
            (before_eq.rfind('['), before_eq.rfind(']'))
        {
            format!(
                "{}.__setitem__({},{})",
                &e[..bracket_start],
                &e[bracket_start + 1..bracket_end],
                after_eq
            )
        } else {
            format!("{}:={}", before_eq, after_eq)
        }
    } else {
        e
    }
}
