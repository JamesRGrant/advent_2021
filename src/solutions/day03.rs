use crate::Solve;

pub struct Problem {
    data: Vec<String>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        let mut g = String::from("");
        let mut e = String::from("");

        // For each position, put the most/least popular digit in a string
        for i in 0..self.data[0].len() {
            if count_most(&self.data, i) == 1 {
                g.push('1');
                e.push('0');
            } else {
                g.push('0');
                e.push('1');
            }
        }
        let e_val = i64::from_str_radix(e.as_str(), 2).unwrap();
        let g_val = i64::from_str_radix(g.as_str(), 2).unwrap();
        e_val * g_val
    }
    fn p2(&mut self) -> i64 {
        let e_val = remove_match(&mut self.data.clone(), '1', '0');
        let g_val = remove_match(&mut self.data.clone(), '0', '1');
        e_val * g_val
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        Problem {
            data: input.to_vec(),
        }
    }
}

fn count_most(v: &[String], pos: usize) -> i8 {
    let mut ones = 0;
    let mut zeros = 0;

    for line in v.iter() {
        match line.chars().nth(pos).unwrap() {
            '1' => ones += 1,
            '0' => zeros += 1,
            _ => panic!("Unexpected non binary digit!"),
        }
    }

    match ones.cmp(&zeros) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Equal => -1,
        std::cmp::Ordering::Less => 0,
    }
}

fn remove_match(v: &mut Vec<String>, remove_match: char, remove_other: char) -> i64 {
    let len = v[0].len();
    let mut remove;

    // Go position by position
    // Keep only the numbers that have the popular digit
    // Repeat until only one remains
    for i in 0..len {
        if count_most(v, i) == 0 {
            remove = remove_match;
        } else {
            remove = remove_other; // Tie keeps this one
        }
        let mut j = 0;
        while j < v.len() {
            if v[j].chars().nth(i).unwrap() == remove {
                v.swap_remove(j);
            } else {
                // Only move on if you did not swap
                // Otherwise, you need to check what was just swapped in
                j += 1;
            }
        }

        if v.len() == 1 {
            break;
        }
    }
    i64::from_str_radix(v[0].as_str(), 2).unwrap()
}
