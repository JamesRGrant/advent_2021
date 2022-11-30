use crate::Solve;

pub struct Problem {
    data: Vec<String>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        let mut retval: i64 = 0;
        let mut cmd: Vec<char> = Vec::new();

        for line in &self.data {
            retval += score_command(line, &mut cmd);
        }
        retval
    }
    fn p2(&mut self) -> i64 {
        let mut retval: i64;
        let mut scores: Vec<i64> = Vec::new();
        let mut cmd: Vec<char> = Vec::new();

        for line in &self.data {
            retval = score_command(line, &mut cmd);
            if retval == 0 {
                while !cmd.is_empty() {
                    match cmd.pop().unwrap() {
                        '(' => retval = retval * 5 + 1,
                        '[' => retval = retval * 5 + 2,
                        '{' => retval = retval * 5 + 3,
                        '<' => retval = retval * 5 + 4,
                        _ => (),
                    }
                }
                scores.push(retval);
            }
        }
        scores.sort_unstable();
        scores[(scores.len() - 1) / 2]
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        Problem {
            data: input.to_vec(),
        }
    }
}

pub fn score_command(input: &str, cmd: &mut Vec<char>) -> i64 {
    let mut retval: i64 = 0;
    cmd.clear();
    for c in input.chars() {
        match c {
            '(' | '[' | '{' | '<' => cmd.push(c),
            ')' => {
                if cmd.pop() != Some('(') {
                    retval += 3;
                    break;
                }
            }
            ']' => {
                if cmd.pop() != Some('[') {
                    retval += 57;
                    break;
                }
            }
            '}' => {
                if cmd.pop() != Some('{') {
                    retval += 1197;
                    break;
                }
            }
            '>' => {
                if cmd.pop() != Some('<') {
                    retval += 25137;
                    break;
                }
            }
            _ => (),
        }
    }
    retval
}
