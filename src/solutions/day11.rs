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

pub fn algorithm1(v: &[String]) -> i64 {
    let mut a2d: Vec<Vec<i64>> = Vec::new();
    let mut retval: i64 = 0;
    let rows: usize = v.len();
    let cols: usize = v[0].len();
    for line in v {
        let row: Vec<i64> = line.chars().flat_map(|c| c.to_string().parse()).collect();
        a2d.push(row);
    }

    for _i in 0..100 {
        let mut flashes: Vec<Vec<i64>> = vec![vec![0; cols]; rows];
        increment_all(&mut a2d);
        while check_for_flash(&mut a2d, &mut flashes) {}
        reset_flashes(&mut a2d);
        retval += count_flashes(&flashes);
    }

    retval
}

pub fn algorithm2(v: &[String]) -> i64 {
    let mut a2d: Vec<Vec<i64>> = Vec::new();
    let retval: i64;
    let rows: usize = v.len();
    let cols: usize = v[0].len();
    for line in v {
        let row: Vec<i64> = line.chars().flat_map(|c| c.to_string().parse()).collect();
        a2d.push(row);
    }
    let mut i: i64 = 0;
    loop {
        let mut flashes: Vec<Vec<i64>> = vec![vec![0; cols]; rows];
        increment_all(&mut a2d);
        while check_for_flash(&mut a2d, &mut flashes) {}
        reset_flashes(&mut a2d);

        if count_flashes(&flashes) == rows as i64 * cols as i64 {
            retval = i + 1;
            break;
        }
        i += 1;
    }

    retval
}

pub fn increment_all(v: &mut Vec<Vec<i64>>) {
    for row in v {
        for col in row {
            *col += 1;
        }
    }
}

pub fn check_for_flash(v: &mut Vec<Vec<i64>>, f: &mut Vec<Vec<i64>>) -> bool {
    let mut retval: bool = false;
    for row in 0..v.len() {
        for col in 0..v[row].len() {
            if v[row][col] > 9 && f[row][col] == 0 {
                f[row][col] = 1;
                flash(v, row, col);
                retval = true;
            }
        }
    }

    retval
}

pub fn flash(v: &mut Vec<Vec<i64>>, row: usize, col: usize) {
    let rows = v.len();
    let cols = v[0].len();

    // Start left, go clockwise
    if col > 0 {
        v[row][col - 1] += 1;
    }
    if col > 0 && row > 0 {
        v[row - 1][col - 1] += 1;
    }
    if row > 0 {
        v[row - 1][col] += 1;
    }
    if row > 0 && col < cols - 1 {
        v[row - 1][col + 1] += 1;
    }
    if col < cols - 1 {
        v[row][col + 1] += 1;
    }
    if row < rows - 1 && col < cols - 1 {
        v[row + 1][col + 1] += 1;
    }
    if row < rows - 1 {
        v[row + 1][col] += 1;
    }
    if row < rows - 1 && col > 0 {
        v[row + 1][col - 1] += 1;
    }
}

pub fn count_flashes(v: &[Vec<i64>]) -> i64 {
    let mut retval: i64 = 0;
    for row in v {
        for col in row {
            if *col == 1 {
                retval += 1;
            }
        }
    }
    retval
}

pub fn reset_flashes(v: &mut [Vec<i64>]) {
    for row in v {
        for col in row {
            if *col > 9 {
                *col = 0;
            }
        }
    }
}
