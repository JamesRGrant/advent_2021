use crate::Solve;

pub struct Problem {
    data: Vec<String>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        let input: [i64; 14] = [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9];
        // let mut last = 9;

        // while algorithm2(&input, v) != 0 {
        //     if last != input[5] {
        //         println!("{}", array_to_number(&input));
        //         last = input[5];
        //     }
        //     create_next_number(&mut input, 13);
        // }

        array_to_number(&input)
    }
    fn p2(&mut self) -> i64 {
        let input: [i64; 14] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

        algorithm2(&input, &self.data)
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        Problem {
            data: input.to_vec(),
        }
    }
}

#[allow(dead_code)]
fn array_to_number(input: &[i64; 14]) -> i64 {
    let mut output = 0;
    let mut pos = 1;
    for i in (0..14).rev() {
        output += pos * input[i];
        pos *= 10;
    }
    output
}

#[allow(dead_code)]
fn create_next_number(input: &mut [i64; 14], i: usize) {
    input[i] -= 1;
    if input[i] == 0 {
        input[i] = 9;
        create_next_number(input, i - 1);
    }
}

fn algorithm2(input: &[i64; 14], v: &[String]) -> i64 {
    let mut c1: [i64; 14] = [0; 14];
    let mut c2: [i64; 14] = [0; 14];
    let mut c3: [i64; 14] = [0; 14];

    for i in 0..14 {
        c1[i] = v[i * 18 + 4]
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<String>>()[2]
            .parse()
            .unwrap();
        c2[i] = v[i * 18 + 5]
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<String>>()[2]
            .parse()
            .unwrap();
        c3[i] = v[i * 18 + 15]
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<String>>()[2]
            .parse()
            .unwrap();
    }

    let mut z = 0;
    for i in 0..14 {
        algorithm2_1(input[i], &mut z, c1[i], c2[i], c3[i]);
    }
    z
}

fn algorithm2_1(w: i64, z: &mut i64, c1: i64, c2: i64, c3: i64) {
    let x = if ((*z % 26) + c2) == w { 0 } else { 1 };
    *z = *z / c1 * ((25 * x) + 1);
    *z += (w + c3) * x;
    //println!("  z = {}", *z);
}

#[allow(dead_code)]
fn algorithm1(pgm: &[String], input: &[i64; 14]) -> i64 {
    let mut cur_input = 0;
    let mut w: i64 = 0;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut z: i64 = 0;

    for instr in pgm {
        let opers: Vec<String> = instr.split_whitespace().map(str::to_string).collect();
        // First operand is always a variable
        let tmp;

        // println!("{}", instr);
        if opers[0].as_str() == "inp" {
            match opers[1].as_str() {
                "w" => tmp = &mut w,
                "x" => tmp = &mut x,
                "y" => tmp = &mut y,
                "z" => tmp = &mut z,
                _ => panic!("Bad instruction"),
            }
            *tmp = input[cur_input];
            cur_input += 1;
            // println!("  inp {} = {}", opers[1], *tmp);
        } else {
            let tmp2 = match opers[2].as_str() {
                "w" => w,
                "x" => x,
                "y" => y,
                "z" => z,
                _ => opers[2].parse().unwrap(),
            };
            match opers[1].as_str() {
                "w" => tmp = &mut w,
                "x" => tmp = &mut x,
                "y" => tmp = &mut y,
                "z" => tmp = &mut z,
                _ => panic!("Bad instruction"),
            }
            match opers[0].as_str() {
                "add" => *tmp += tmp2,
                "mul" => *tmp *= tmp2,
                "div" => *tmp /= tmp2,
                "mod" => *tmp %= tmp2,
                "eql" => {
                    if *tmp == tmp2 {
                        *tmp = 1;
                    } else {
                        *tmp = 0;
                    }
                }
                _ => panic!("Bad instruction"),
            }

            // println!("  {} = {} from {} {}", opers[1], *tmp, opers[0], tmp2);
            // println!("[{}, {}, {}, {}]", w, x, y, z);
        }
    }

    z
}
