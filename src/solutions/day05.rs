use crate::Solve;
use std::collections::HashMap;

pub struct Problem {
    data: Vec<String>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        count_multi_hits(&self.data, false)
    }
    fn p2(&mut self) -> i64 {
        count_multi_hits(&self.data, true)
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        Problem {
            data: input.to_vec(),
        }
    }
}

struct Line {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

fn count_multi_hits(v: &[String], include_diags: bool) -> i64 {
    let mut retval: i64 = 0;
    let points = load_points(v);
    let mut point_map: HashMap<(i64, i64), i64> = HashMap::new();

    for line in &points {
        draw_line(line, &mut point_map, include_diags);
    }
    for value in point_map.values() {
        if value > &1 {
            retval += 1;
        }
    }
    retval
}

fn load_points(v: &[String]) -> Vec<Line> {
    let mut points: Vec<Line> = Vec::new();

    for line in v.iter() {
        let num_string = line.replace(" -> ", ",");
        let nums: Vec<i64> = num_string.split(',').map(|s| s.parse().unwrap()).collect();
        points.push(Line {
            x1: nums[0],
            y1: nums[1],
            x2: nums[2],
            y2: nums[3],
        });
    }
    points
}

fn draw_line(line: &Line, point_map: &mut HashMap<(i64, i64), i64>, include_diag: bool) {
    let mut x = line.x1;
    let mut y = line.y1;
    let x_inc: i64;
    let y_inc: i64;
    let mut len: i64 = 1;

    match line.x1.cmp(&line.x2) {
        std::cmp::Ordering::Equal => x_inc = 0,
        std::cmp::Ordering::Less => {
            x_inc = 1;
            len = line.x2 - line.x1 + 1;
        }
        std::cmp::Ordering::Greater => {
            x_inc = -1;
            len = line.x1 - line.x2 + 1;
        }
    }

    match line.y1.cmp(&line.y2) {
        std::cmp::Ordering::Equal => y_inc = 0,
        std::cmp::Ordering::Less => {
            y_inc = 1;
            len = line.y2 - line.y1 + 1;
        }
        std::cmp::Ordering::Greater => {
            y_inc = -1;
            len = line.y1 - line.y2 + 1;
        }
    }

    if x_inc == 0 || y_inc == 0 || include_diag {
        for _i in 0..len {
            *point_map.entry((x, y)).or_insert(0) += 1;
            x += x_inc;
            y += y_inc;
        }
    }
}
