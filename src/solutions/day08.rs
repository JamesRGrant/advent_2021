use crate::Solve;

pub struct Problem {
    data: Vec<String>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        let mut y: Vec<Vec<String>> = Vec::new();
        for line in &self.data {
            let tmp: Vec<String> = line.split('|').map(str::to_string).collect();
            y.push(tmp[1].split_whitespace().map(str::to_string).collect());
        }
        algorithm_1(y)
    }
    fn p2(&mut self) -> i64 {
        let mut retval = 0;
        for line in &self.data {
            let tmp: Vec<String> = line.split('|').map(str::to_string).collect();
            retval += algorithm_2(
                tmp[0]
                    .split_whitespace()
                    .map(str::to_string)
                    .collect::<Vec<String>>()
                    .as_slice(),
                tmp[1]
                    .split_whitespace()
                    .map(str::to_string)
                    .collect::<Vec<String>>()
                    .as_slice(),
            );
        }
        retval
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        Problem {
            data: input.to_vec(),
        }
    }
}

fn algorithm_1(v: Vec<Vec<String>>) -> i64 {
    let mut retval: i64 = 0;

    for line in v {
        for output in line {
            let len = output.len();
            if len == 2 || len == 3 || len == 4 || len == 7 {
                retval += 1;
            }
        }
    }
    retval
}

// Sample: be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
//  000
// 5   1
//  666
// 4   2
//  333
fn algorithm_2(left: &[String], right: &[String]) -> i64 {
    let mut retval: i64 = 0;
    let mut segments: Vec<char> = vec![' '; 7];
    let mut words: Vec<String> = vec!["".to_string(); 10];
    let mut five_words: Vec<String> = Vec::new();
    let mut six_words: Vec<String> = Vec::new();
    let mut left_sorted: Vec<String> = Vec::new();
    let mut right_sorted: Vec<String> = Vec::new();

    // Sort every string alphabetically
    for word in left {
        let mut s: Vec<char> = word.chars().collect();
        s.sort_unstable();
        left_sorted.push(s.iter().collect());
    }
    for word in right {
        let mut s: Vec<char> = word.chars().collect();
        s.sort_unstable();
        right_sorted.push(s.iter().collect());
    }

    for word in &left_sorted {
        // Map easy to identify digits direct
        match word.len() {
            2 => words[1] = word.to_string(),
            3 => words[7] = word.to_string(),
            4 => words[4] = word.to_string(),
            7 => words[8] = word.to_string(),
            5 => five_words.push(word.to_string()),
            6 => six_words.push(word.to_string()),
            _ => (),
        }
    }

    find_segments(
        &mut words,
        &mut segments,
        five_words.as_slice(),
        six_words.as_slice(),
    );
    // finish words
    finish_words(&mut words, &segments);

    // Sort every word alphabetically
    for word in &mut words {
        let mut s: Vec<char> = word.chars().collect();
        s.sort_unstable();
        *word = s.iter().collect();
    }

    let mut multiplier: i64 = 1000;

    for w in &right_sorted {
        for (i, word) in words.iter().enumerate() {
            if *w == *word {
                retval += multiplier * i as i64;
            }
        }
        multiplier /= 10;
    }

    retval
}

fn find_segments(
    words: &mut Vec<String>,
    segments: &mut Vec<char>,
    five_words: &[String],
    six_words: &[String],
) {
    // Find segment 0
    for c in words[7].chars() {
        if !words[1].contains(c) {
            segments[0] = c;
        }
    }

    // Find segments 6 and 3
    let mut char_count: Vec<i32> = vec![0; 7];
    for w in five_words.iter() {
        for c in w.chars() {
            match c {
                'a' => char_count[0] += 1,
                'b' => char_count[1] += 1,
                'c' => char_count[2] += 1,
                'd' => char_count[3] += 1,
                'e' => char_count[4] += 1,
                'f' => char_count[5] += 1,
                'g' => char_count[6] += 1,
                _ => (),
            }
        }
    }
    let mut middles: Vec<char> = Vec::new();
    for (i, count) in char_count.iter().enumerate() {
        if *count == 3 {
            match i {
                0 => middles.push('a'),
                1 => middles.push('b'),
                2 => middles.push('c'),
                3 => middles.push('d'),
                4 => middles.push('e'),
                5 => middles.push('f'),
                6 => middles.push('g'),
                _ => (),
            }
        }
    }
    for c in &middles {
        if *c != segments[0] {
            if words[4].contains(*c) {
                segments[6] = *c;
            } else {
                segments[3] = *c;
            }
        }
    }

    // Find segment 5
    for c in words[4].chars() {
        if !words[1].contains(c) && c != segments[6] {
            segments[5] = c;
        }
    }

    // Find segemnt 1, then 2
    for c in words[1].chars() {
        for w in six_words {
            if !w.contains(c) {
                segments[1] = c;
                for c2 in words[1].chars() {
                    if c2 != c {
                        segments[2] = c2;
                    }
                }
            }
        }
    }

    // Finally find segemnt 4
    for c in words[8].chars() {
        if c != segments[0]
            && c != segments[1]
            && c != segments[2]
            && c != segments[3]
            && c != segments[5]
            && c != segments[6]
        {
            segments[4] = c;
        }
    }
}

fn finish_words(words: &mut Vec<String>, segments: &[char]) {
    words[0].push(segments[0]);
    words[0].push(segments[1]);
    words[0].push(segments[2]);
    words[0].push(segments[3]);
    words[0].push(segments[4]);
    words[0].push(segments[5]);

    words[2].push(segments[0]);
    words[2].push(segments[1]);
    words[2].push(segments[3]);
    words[2].push(segments[4]);
    words[2].push(segments[6]);

    words[3].push(segments[0]);
    words[3].push(segments[1]);
    words[3].push(segments[2]);
    words[3].push(segments[3]);
    words[3].push(segments[6]);

    words[5].push(segments[0]);
    words[5].push(segments[2]);
    words[5].push(segments[3]);
    words[5].push(segments[5]);
    words[5].push(segments[6]);

    words[6].push(segments[0]);
    words[6].push(segments[2]);
    words[6].push(segments[3]);
    words[6].push(segments[4]);
    words[6].push(segments[5]);
    words[6].push(segments[6]);

    words[9].push(segments[0]);
    words[9].push(segments[1]);
    words[9].push(segments[2]);
    words[9].push(segments[3]);
    words[9].push(segments[5]);
    words[9].push(segments[6]);
}
