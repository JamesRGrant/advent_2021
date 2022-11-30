use crate::Solve;

pub struct Problem {
    data: Vec<String>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        algorithm1(&self.data)
    }
    fn p2(&mut self) -> i64 {
        algorithm2(&self.data)
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        Problem {
            data: input.to_vec(),
        }
    }
}
fn algorithm1(v: &[String]) -> i64 {
    let mut s = load_line(&v[0]);

    for line in v.iter().skip(1) {
        let tmp = load_line(line);
        s.add(tmp);
        while s.explode(0).0 || s.split() {}
    }
    s.magnitude()
}

fn algorithm2(v: &[String]) -> i64 {
    let mut retval = 0;
    for i in 0..v.len() {
        for j in 0..v.len() {
            if j != i {
                let mut s = load_line(&v[i]);
                let t = load_line(&v[j]);
                s.add(t);
                while s.explode(0).0 || s.split() {}
                retval = retval.max(s.magnitude());
            }
        }
    }
    retval
}

fn load_line(input: &str) -> SnailNum {
    let mut s = SnailNum {
        lval: 0,
        rval: 0,
        left: None,
        right: None,
    };
    s.parse(input, 0);
    s
}

struct SnailNum {
    lval: i64,
    rval: i64,
    left: Option<Box<SnailNum>>,
    right: Option<Box<SnailNum>>,
}

impl SnailNum {
    fn parse(&mut self, input: &str, index: usize) -> usize {
        let mut i = index;
        assert!(
            &input[i..=i] == "[",
            "SnailNum::parse: expected '[': {}",
            &input[i..]
        );

        i += 1;
        if &input[i..=i] == "[" {
            self.left = Some(Box::new(SnailNum {
                lval: 0,
                rval: 0,
                left: None,
                right: None,
            }));

            i = self.left.as_mut().unwrap().parse(input, i);
        } else {
            self.lval = input[i..=i].parse().unwrap();
            i += 1;
        }

        assert!(
            &input[i..=i] == ",",
            "SnailNum::parse: expected ',': {}",
            &input[i..],
        );
        i += 1;

        if &input[i..=i] == "[" {
            self.right = Some(Box::new(SnailNum {
                lval: 0,
                rval: 0,
                left: None,
                right: None,
            }));

            i = self.right.as_mut().unwrap().parse(input, i);
        } else {
            self.rval = input[i..=i].parse().unwrap();
            i += 1;
        }

        assert!(
            &input[i..=i] == "]",
            "SnailNum::parse: expected ']': {}",
            &input[i..]
        );
        i += 1;

        i
    }

    #[allow(dead_code)]
    fn as_string(&self) -> String {
        let mut s = String::new();
        s.push('[');
        if self.left.is_some() {
            s += &self.left.as_ref().unwrap().as_string();
        } else {
            s += &self.lval.to_string();
        }
        s.push(',');
        if self.right.is_some() {
            s += &self.right.as_ref().unwrap().as_string();
        } else {
            s += &self.rval.to_string();
        }
        s.push(']');

        s
    }

    fn magnitude(&self) -> i64 {
        let a: i64 = if self.left.is_some() {
            self.left.as_ref().unwrap().magnitude()
        } else {
            self.lval
        };
        let b: i64 = if self.right.is_some() {
            self.right.as_ref().unwrap().magnitude()
        } else {
            self.rval
        };
        (3 * a) + (2 * b)
    }

    fn explode(&mut self, level: usize) -> (bool, i64, i64) {
        let mut retval = false;
        let mut return_left: i64 = 0;
        let mut return_right: i64 = 0;
        if self.left.is_some() {
            if level == 3 {
                self.lval = 0;
                return_left = self.left.as_ref().unwrap().lval;
                return_right = self.left.as_ref().unwrap().rval;
                self.left = None;
                retval = true;
            } else {
                let (a, b, c) = self.left.as_mut().unwrap().explode(level + 1);
                retval = a;
                return_left = b;
                return_right = c;
            }
            if retval && return_right != -1 {
                if self.right.is_none() {
                    self.rval += return_right;
                    return_right = -1;
                } else if self.right.as_mut().unwrap().apply_right(return_right) {
                    return_right = -1;
                }
            }
        }

        if self.right.is_some() && !retval {
            if level == 3 {
                self.rval = 0;
                return_left = self.right.as_ref().unwrap().lval;
                return_right = self.right.as_ref().unwrap().rval;
                self.right = None;
                retval = true;
            } else {
                let (a, b, c) = self.right.as_mut().unwrap().explode(level + 1);
                retval = a;
                return_left = b;
                return_right = c;
            }
            if retval && return_left != -1 {
                if self.left.is_none() {
                    self.lval += return_left;
                    return_left = -1;
                } else if self.left.as_mut().unwrap().apply_left(return_left) {
                    return_left = -1;
                }
            }
        }

        (retval, return_left, return_right)
    }

    fn split(&mut self) -> bool {
        let mut retval = false;
        if self.left.is_some() {
            retval = self.left.as_mut().unwrap().split();
        } else if self.lval > 9 {
            self.left = Some(Box::new(SnailNum {
                lval: self.lval / 2,
                rval: self.lval / 2 + self.lval % 2,
                left: None,
                right: None,
            }));
            self.lval = 0;
            retval = true;
        }
        if self.right.is_some() && !retval {
            retval = self.right.as_mut().unwrap().split();
        } else if !retval && self.rval > 9 {
            self.right = Some(Box::new(SnailNum {
                lval: self.rval / 2,
                rval: self.rval / 2 + self.rval % 2,
                left: None,
                right: None,
            }));
            self.rval = 0;
            retval = true;
        }
        retval
    }

    fn apply_right(&mut self, val: i64) -> bool {
        let mut retval = if self.left.is_none() {
            self.lval += val;
            true
        } else {
            self.left.as_mut().unwrap().apply_right(val)
        };
        if self.right.is_none() && !retval {
            self.rval += val;
            retval = true;
        } else if !retval {
            retval = self.right.as_mut().unwrap().apply_right(val);
        }
        retval
    }

    fn apply_left(&mut self, val: i64) -> bool {
        let mut retval = if self.right.is_none() {
            self.rval += val;
            true
        } else {
            self.right.as_mut().unwrap().apply_left(val)
        };
        if self.left.is_none() && !retval {
            self.lval += val;
            retval = true;
        } else if !retval {
            retval = self.left.as_mut().unwrap().apply_right(val);
        }
        retval
    }

    fn add(&mut self, rhs: SnailNum) {
        let lhs = SnailNum {
            lval: self.lval,
            rval: self.rval,
            left: self.left.take(),
            right: self.right.take(),
        };
        self.lval = 0;
        self.rval = 0;
        self.left = Some(Box::new(lhs));
        self.right = Some(Box::new(rhs));
    }
}
