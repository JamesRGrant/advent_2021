use multimap::MultiMap;
use std::collections::HashMap;

use crate::Solve;

pub struct Problem {
    data: Vec<String>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        a1(&self.data)
    }
    fn p2(&mut self) -> i64 {
        a2(&self.data)
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        Problem {
            data: input.to_vec(),
        }
    }
}

fn a1(v: &[String]) -> i64 {
    let mut pt: Vec<char> = v[0].chars().collect();
    let m: HashMap<(char, char), char> = load_data(v);

    algorithm2(&mut pt, &m, 10)
}

fn a2(v: &[String]) -> i64 {
    // Count the pairs in the input string
    let mut pairs: HashMap<String, i64> = HashMap::new();
    for i in 0..v[0].len() - 1 {
        let mut tmp: String = String::new();
        tmp.push(v[0].chars().nth(i).unwrap());
        tmp.push(v[0].chars().nth(i + 1).unwrap());
        *pairs.entry(tmp).or_insert(0) += 1;
    }

    // Load a map of pair to 2 pairs
    let mut m: MultiMap<String, String> = MultiMap::new();
    for line in v.iter().skip(2) {
        let src: String = line[0..2].to_string();
        let mut t1: String = String::new();
        let mut t2: String = String::new();
        t1.push(line.chars().next().unwrap());
        t1.push(line.chars().nth(6).unwrap());
        t2.push(line.chars().nth(6).unwrap());
        t2.push(line.chars().nth(1).unwrap());
        m.insert(src.clone(), t1);
        m.insert(src.clone(), t2);
    }

    score3(
        algorithm3(&pairs, m, 0, 40),
        v[0].chars().next().unwrap(),
        v[0].chars().nth(v[0].len() - 1).unwrap(),
    )
}

pub fn load_data(v: &[String]) -> HashMap<(char, char), char> {
    let mut m: HashMap<(char, char), char> = HashMap::new();
    for line in v.iter().skip(2) {
        m.insert(
            (line.chars().next().unwrap(), line.chars().nth(1).unwrap()),
            line.chars().nth(6).unwrap(),
        );
    }
    m
}

#[allow(dead_code)]
fn algorithm1(pt: &mut Vec<char>, m: &HashMap<(char, char), char>, loops: usize) -> i64 {
    // Just insert the characters into the string
    for _i in 0..loops {
        let mut j = 0;
        while j < pt.len() - 1 {
            pt.insert(j + 1, m[&(pt[j], pt[j + 1])]);
            j += 2;
        }
    }
    let mut scores: HashMap<char, i64> = HashMap::new();

    for c in pt {
        *scores.entry(*c).or_insert(0) += 1;
    }
    let vals: Vec<i64> = scores.into_values().collect();
    vals.iter().max().unwrap() - vals.iter().min().unwrap()
}

fn algorithm2(pt: &mut Vec<char>, m: &HashMap<(char, char), char>, loops: usize) -> i64 {
    let mut char_count: Vec<i64> = vec![0; 26];

    // Recurse, counting as we go
    char_count[pt[0] as usize - 65] += 1;
    for i in 0..pt.len() - 1 {
        char_count[pt[i + 1] as usize - 65] += 1;
        spawn(pt[i], pt[i + 1], &mut char_count, m, 0, loops);
    }
    // this is bad if there are zeros!
    char_count.iter().max().unwrap() - char_count.iter().min().unwrap()
}

fn spawn(
    a: char,
    b: char,
    char_count: &mut Vec<i64>,
    m: &HashMap<(char, char), char>,
    cur: usize,
    max: usize,
) {
    let c = m[&(a, b)];
    char_count[c as usize - 65] += 1;
    if cur + 1 < max {
        spawn(a, c, char_count, m, cur + 1, max);
        spawn(c, b, char_count, m, cur + 1, max);
    }
}

fn algorithm3(
    pairs: &HashMap<String, i64>,
    m: MultiMap<String, String>,
    cur: i32,
    max: i32,
) -> HashMap<String, i64> {
    let mut new_pairs: HashMap<String, i64> = HashMap::new();

    for (key, val) in pairs {
        let targets: &Vec<String> = &*m.get_vec(key).unwrap();
        *new_pairs.entry(targets[0].clone()).or_insert(0) += val;
        *new_pairs.entry(targets[1].clone()).or_insert(0) += val;
    }

    if cur == max - 1 {
        new_pairs
    } else {
        algorithm3(&new_pairs, m, cur + 1, max)
    }
}

fn score3(new_pairs: HashMap<String, i64>, first: char, last: char) -> i64 {
    let mut char_count: HashMap<char, i64> = HashMap::new();

    *char_count.entry(first).or_insert(0) += 1;
    *char_count.entry(last).or_insert(0) += 1;

    for (key, val) in new_pairs {
        *char_count.entry(key.chars().next().unwrap()).or_insert(0) += val;
        *char_count.entry(key.chars().nth(1).unwrap()).or_insert(0) += val;
    }
    let vals: Vec<i64> = char_count.into_values().collect();
    (vals.iter().max().unwrap() - vals.iter().min().unwrap()) / 2
}
