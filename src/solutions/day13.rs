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
    let mut points: Vec<(u32, u32)> = Vec::new();
    let mut folds: Vec<String> = Vec::new();

    for line in v.iter() {
        if !line.is_empty() {
            if line[0..3].to_string() == "fol" {
                folds.push(line[11..].to_string());
            } else {
                let tmp: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
                points.push((tmp[0], tmp[1]));
            }
        }
    }

    let tmp: Vec<String> = folds[0].split('=').map(str::to_string).collect();
    if tmp[0] == "x" {
        fold_x(&mut points, tmp[1].parse().unwrap());
    } else {
        fold_y(&mut points, tmp[1].parse().unwrap());
    }

    points.len() as i64
}

pub fn algorithm2(v: &[String]) -> i64 {
    let mut points: Vec<(u32, u32)> = Vec::new();
    let mut folds: Vec<String> = Vec::new();

    for line in v.iter() {
        if !line.is_empty() {
            if line[0..3].to_string() == "fol" {
                folds.push(line[11..].to_string());
            } else {
                let tmp: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
                points.push((tmp[0], tmp[1]));
            }
        }
    }

    for fold in folds {
        let tmp: Vec<String> = fold.split('=').map(str::to_string).collect();
        if tmp[0] == "x" {
            fold_x(&mut points, tmp[1].parse().unwrap());
        } else {
            fold_y(&mut points, tmp[1].parse().unwrap());
        }
    }

    print_points(&points);
    0
}

fn fold_y(points: &mut Vec<(u32, u32)>, index: u32) {
    let mut i = 0;
    while i < points.len() {
        if points[i].1 > index {
            let new_point: (u32, u32) = (points[i].0, index - (points[i].1 - index));

            if !points.contains(&new_point) {
                points.push(new_point);
            }
            points.swap_remove(i);
        } else {
            i += 1;
        }
    }
}

fn fold_x(points: &mut Vec<(u32, u32)>, index: u32) {
    let mut i = 0;
    while i < points.len() {
        if points[i].0 > index {
            let new_point: (u32, u32) = (index - (points[i].0 - index), points[i].1);

            if !points.contains(&new_point) {
                points.push(new_point);
            }
            points.swap_remove(i);
        } else {
            i += 1;
        }
    }
}

fn print_points(points: &[(u32, u32)]) {
    let mut rows: Vec<Vec<char>> = vec![vec![' '; 40]; 6];

    for p in points {
        rows[p.1 as usize][p.0 as usize] = '#';
    }

    for r in rows {
        let s: String = r.into_iter().collect();
        println!("      {s}");
    }
}
