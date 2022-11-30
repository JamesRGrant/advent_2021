use crate::Solve;

pub struct Problem {
    data: String,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        let mut j = 0;
        parse_header(&hex_to_binary(&self.data), &mut j, true)
    }
    fn p2(&mut self) -> i64 {
        let mut j = 0;
        parse_header(&hex_to_binary(&self.data), &mut j, false)
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        Problem {
            data: input[0].clone(),
        }
    }
}

pub fn hex_to_binary(input: &str) -> String {
    let mut out = String::new();

    for c in input.chars() {
        match c {
            '0' => out.push_str("0000"),
            '1' => out.push_str("0001"),
            '2' => out.push_str("0010"),
            '3' => out.push_str("0011"),
            '4' => out.push_str("0100"),
            '5' => out.push_str("0101"),
            '6' => out.push_str("0110"),
            '7' => out.push_str("0111"),
            '8' => out.push_str("1000"),
            '9' => out.push_str("1001"),
            'A' => out.push_str("1010"),
            'B' => out.push_str("1011"),
            'C' => out.push_str("1100"),
            'D' => out.push_str("1101"),
            'E' => out.push_str("1110"),
            'F' => out.push_str("1111"),
            _ => panic!("Unexpected input!"),
        }
    }
    out
}

fn parse_header(cmd: &str, i: &mut usize, intro: bool) -> i64 {
    let version = cmd[*i..(*i + 3)].to_string();
    let ver = i64::from_str_radix(version.as_str(), 2).unwrap();
    let ptype = cmd[(*i + 3)..(*i + 6)].to_string();
    *i += 6;
    let mut retval: i64 = 0;

    if ptype == "100" {
        if intro {
            parse_literal(cmd, i);
        } else {
            retval = parse_literal(cmd, i);
        }
    } else {
        if *i > 5300 {
            println!("{}", &cmd[*i..]);
        }
        *i += 1;
        let mut nums: Vec<i64> = Vec::new();
        if &cmd[(*i - 1)..(*i)] == "0" {
            let cmd_bits = usize::from_str_radix(&cmd[*i..(*i + 15)], 2).unwrap() + *i + 15;
            *i += 15;
            while *i != cmd_bits {
                if intro {
                    retval += parse_header(cmd, i, intro);
                } else {
                    nums.push(parse_header(cmd, i, intro));
                }
            }
        } else {
            let cmd_count = usize::from_str_radix(&cmd[*i..(*i + 11)], 2).unwrap();
            *i += 11;
            for _walk in 0..cmd_count {
                if intro {
                    retval += parse_header(cmd, i, intro);
                } else {
                    nums.push(parse_header(cmd, i, intro));
                }
            }
        }

        if !intro {
            match ptype.as_str() {
                "000" => {
                    for n in nums {
                        retval += n;
                    }
                }
                "001" => {
                    retval = 1;
                    for n in nums {
                        retval *= n;
                    }
                }
                "010" => {
                    retval = *nums.iter().min().unwrap();
                }
                "011" => {
                    retval = *nums.iter().max().unwrap();
                }
                "101" => {
                    if nums[0] > nums[1] {
                        retval = 1;
                    } else {
                        retval = 0;
                    }
                }
                "110" => {
                    if nums[0] < nums[1] {
                        retval = 1;
                    } else {
                        retval = 0;
                    }
                }
                "111" => {
                    if nums[0] == nums[1] {
                        retval = 1;
                    } else {
                        retval = 0;
                    }
                }
                _ => {}
            }
        }
    }

    if intro {
        retval + ver
    } else {
        retval
    }
}

fn parse_literal(cmd: &str, i: &mut usize) -> i64 {
    let mut tmp = String::new();
    loop {
        tmp.push_str(&cmd[(*i + 1)..(*i + 5)]);
        *i += 5;
        if &cmd[(*i - 5)..(*i - 4)] == "0" {
            break;
        }
    }
    i64::from_str_radix(tmp.as_str(), 2).unwrap()
}
