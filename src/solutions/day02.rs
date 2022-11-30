use crate::Solve;

pub struct Problem {
    cmds: Vec<(String, i64)>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        let mut h: i64 = 0;
        let mut d: i64 = 0;
        for cmd in &self.cmds {
            match cmd.0.as_str() {
                "forward" => h += cmd.1,
                "down" => d += cmd.1,
                "up" => d -= cmd.1,
                _ => panic!("Unrecognzied command in input."),
            }
        }
        h * d
    }
    fn p2(&mut self) -> i64 {
        let mut h: i64 = 0;
        let mut d: i64 = 0;
        let mut aim: i64 = 0;

        for cmd in &self.cmds {
            match cmd.0.as_str() {
                "forward" => {
                    h += cmd.1;
                    d += aim * cmd.1;
                }
                "down" => aim += cmd.1,
                "up" => aim -= cmd.1,
                _ => panic!("Unrecognzied command in input."),
            }
        }
        h * d
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        let mut cmds: Vec<(String, i64)> = Vec::new();
        for line in input {
            let cmd: Vec<String> = line.split_whitespace().map(str::to_string).collect();
            cmds.push((cmd[0].clone(), cmd[1].parse::<i64>().unwrap()));
        }
        Problem { cmds }
    }
}
