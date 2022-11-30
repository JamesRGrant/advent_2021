use crate::Solve;

pub struct Problem {
    data: Vec<Vec<i64>>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        algorithm_1(&self.data)
    }
    fn p2(&mut self) -> i64 {
        algorithm_2(&self.data)
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        let mut a2d: Vec<Vec<i64>> = Vec::new();
        for line in input {
            let row: Vec<i64> = line.chars().flat_map(|c| c.to_string().parse()).collect();
            a2d.push(row);
        }
        Problem { data: a2d }
    }
}

fn algorithm_1(v: &[Vec<i64>]) -> i64 {
    let mut result: i64 = 0;
    for row in 0..v.len() {
        for col in 0..v[row].len() {
            // Left, Top, Right, Bottom
            if (col == 0 || v[row][col] < v[row][col - 1])
                && (row == 0 || v[row][col] < v[row - 1][col])
                && (col == v[row].len() - 1 || v[row][col] < v[row][col + 1])
                && (row == v.len() - 1 || v[row][col] < v[row + 1][col])
            {
                result += v[row][col] + 1;
            }
        }
    }
    result
}

fn algorithm_2(v: &[Vec<i64>]) -> i64 {
    let mut basin_sizes: Vec<i64> = Vec::new();
    let mut retval: i64 = 1;

    for row in 0..v.len() {
        for col in 0..v[row].len() {
            // Left, Top, Right, Bottom
            if (col == 0 || v[row][col] < v[row][col - 1])
                && (row == 0 || v[row][col] < v[row - 1][col])
                && (col == v[row].len() - 1 || v[row][col] < v[row][col + 1])
                && (row == v.len() - 1 || v[row][col] < v[row + 1][col])
            {
                basin_sizes.push(get_basin_size(v, row, col));
            }
        }
    }

    basin_sizes.sort_unstable();
    for basin_size in basin_sizes.iter().skip(basin_sizes.len() - 3) {
        retval *= basin_size;
    }

    retval
}

fn get_basin_size(v: &[Vec<i64>], row: usize, col: usize) -> i64 {
    let mut points: Vec<(usize, usize)> = Vec::new();
    log_point(v, row, col, &mut points);
    points.len() as i64
}

fn log_point(v: &[Vec<i64>], row: usize, col: usize, points: &mut Vec<(usize, usize)>) {
    if !points.contains(&(row, col)) {
        points.push((row, col));

        // Left, Top, Right, Bottom
        if col > 0 && v[row][col - 1] != 9 {
            log_point(v, row, col - 1, points);
        }
        if row > 0 && v[row - 1][col] != 9 {
            log_point(v, row - 1, col, points);
        }
        if col < v[0].len() - 1 && v[row][col + 1] != 9 {
            log_point(v, row, col + 1, points);
        }
        if row < v.len() - 1 && v[row + 1][col] != 9 {
            log_point(v, row + 1, col, points);
        }
    }
}
