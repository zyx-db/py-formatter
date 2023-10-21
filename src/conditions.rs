use crate::formatter::Formatter;

impl Formatter {
    pub fn compress_condition(&self, mut i: usize) -> (usize, String) {
        let mut if_clause;
        (i, if_clause) = self.get_if_clause(i);

        let mut if_clauses = vec![if_clause];
        let mut else_clause = "None".to_owned();

        while self.lines[i].starts_with("elif") {
            (i, if_clause) = self.get_if_clause(i);
            if_clauses.push(if_clause);
        }

        if self.lines[i].starts_with("else") {
            let block_end = self.block_ends[&i];
            else_clause = self.compress(i + 1, block_end);
            i = block_end;
        }

        (i, clauses_to_str(if_clauses, else_clause))
    }

    fn get_if_clause(&self, i: usize) -> (usize, (String, String)) {
        let block_end = self.block_ends[&i];
        (
            block_end,
            (
                get_boolean_expression(self.lines[i].clone()),
                self.compress(i + 1, block_end),
            ),
        )
    }
}

fn get_boolean_expression(line: String) -> String {
    if line.starts_with("if") {
        &line[2..line.len() - 1]
    } else {
        &line[4..line.len() - 1]
    }
    .trim()
    .to_owned()
}

fn clauses_to_str(if_clauses: Vec<(String, String)>, else_clause: String) -> String {
    if_clauses
        .iter()
        .rev()
        .fold(else_clause, |prev, (condition, block)| {
            format!("[lambda:{},lambda:{}][{}]()", prev, block, condition)
        })
}
