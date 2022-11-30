use crate::Solve;

pub struct Problem {
    data: Vec<String>,
    algo: Vec<bool>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        algorithm1(&self.data, &self.algo, 2)
    }
    fn p2(&mut self) -> i64 {
        algorithm1(&self.data, &self.algo, 50)
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        let mut algo: Vec<bool> = vec![false; input[0].len()];
        for (i, c) in input[0].chars().enumerate() {
            if c == '#' {
                algo[i] = true;
            }
        }

        Problem {
            data: input.to_vec(),
            algo,
        }
    }
}

pub fn algorithm1(v: &[String], algo: &[bool], iter: usize) -> i64 {
    let mut image = load_image(v, iter);
    let max_image = image.len() - iter;
    let mut count = 0;

    for _i in 0..iter {
        image = apply_algo(algo, &image);
    }
    for img in image.iter().take(max_image).skip(iter) {
        count += count_ones(&img[iter..max_image]);
    }
    count as i64
}

fn load_image(v: &[String], iterations: usize) -> Vec<Vec<bool>> {
    let mut img: Vec<Vec<bool>> = Vec::new();
    let length = v[2].len();
    let buffer_length = length + (4 * iterations);

    for _i in 0..(2 * iterations) {
        img.push(vec![false; buffer_length]);
    }
    for line in v.iter().skip(2) {
        let mut x = vec![false; buffer_length];
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                x[i + (2 * iterations)] = true;
            }
        }
        img.push(x);
    }
    for _i in 0..(2 * iterations) {
        img.push(vec![false; buffer_length]);
    }
    img
}

fn apply_algo(algo: &[bool], image: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut new_image: Vec<Vec<bool>> = image.to_vec();

    for row in 0..(image.len() - 2) {
        for col in 0..(image[0].len() - 2) {
            let mut indexer: usize = 0;
            if image[row][col] {
                indexer += 256;
            }
            if image[row][col + 1] {
                indexer += 128;
            }
            if image[row][col + 2] {
                indexer += 64;
            }
            if image[row + 1][col] {
                indexer += 32;
            }
            if image[row + 1][col + 1] {
                indexer += 16;
            }
            if image[row + 1][col + 2] {
                indexer += 8;
            }
            if image[row + 2][col] {
                indexer += 4;
            }
            if image[row + 2][col + 1] {
                indexer += 2;
            }
            if image[row + 2][col + 2] {
                indexer += 1;
            }
            new_image[row + 1][col + 1] = algo[indexer];
        }
    }

    new_image
}

fn count_ones(vb: &[bool]) -> i64 {
    let mut result: i64 = 0;
    for b in vb {
        if *b {
            result += 1;
        }
    }
    result
}
