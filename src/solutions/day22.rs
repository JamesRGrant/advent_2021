use crate::Solve;
// use std::collections::HashSet;

pub struct Problem {
    data: Vec<String>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        let mut map = vec![vec!(vec!(false; 100); 100); 100];

        for line in &self.data {
            let (valid, on, x, y, z) = readline(line);
            if valid {
                mark_cube(&mut map, on, x, y, z);
            }
        }
        count_on(&mut map)
    }
    fn p2(&mut self) -> i64 {
        let mut cubes: Vec<Cube> = Vec::new();
        for line in &self.data {
            process_cube(line, &mut cubes);
        }
        0
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        Problem {
            data: input.to_vec(),
        }
    }
}

type Bound = (usize, usize);

#[allow(dead_code)]
fn readline(line: &str) -> (bool, bool, Bound, Bound, Bound) {
    let mut bounds: Vec<i32> = Vec::new();
    let on = line.chars().nth(1).unwrap() == 'n';
    let tmp: Vec<String> = line.split(' ').map(str::to_string).collect();
    let xyz: Vec<String> = tmp[1].split(',').map(str::to_string).collect();
    for bound in xyz.iter().take(3) {
        let range: Vec<String> = bound.split('=').map(str::to_string).collect();
        let nums: Vec<i32> = range[1].split("..").map(|s| s.parse().unwrap()).collect();
        bounds.push(nums[0]);
        bounds.push(nums[1]);
    }
    let mut valid: bool = true;
    for x in &bounds {
        if *x < -50 || *x > 50 {
            valid = false;
        }
    }

    (
        valid,
        on,
        (
            (std::cmp::min(bounds[0], bounds[1]) + 50).abs() as usize,
            (std::cmp::max(bounds[0], bounds[1]) + 50).abs() as usize,
        ),
        (
            (std::cmp::min(bounds[2], bounds[3]) + 50).abs() as usize,
            (std::cmp::max(bounds[2], bounds[3]) + 50).abs() as usize,
        ),
        (
            (std::cmp::min(bounds[4], bounds[5]) + 50).abs() as usize,
            (std::cmp::max(bounds[4], bounds[5]) + 50).abs() as usize,
        ),
    )
}

#[allow(dead_code)]
fn mark_cube(map: &mut Vec<Vec<Vec<bool>>>, on: bool, x: Bound, y: Bound, z: Bound) {
    // for i in x.0..=x.1 {
    for i in map.iter_mut().take(x.1 + 1).skip(x.0) {
        // for j in y.0..=y.1 {
        for j in i.iter_mut().take(y.1 + 1).skip(y.0) {
            // for k in z.0..=z.1 {
            for k in j.iter_mut().take(z.1 + 1).skip(z.0) {
                *k = on;
            }
        }
    }
}

#[allow(dead_code)]
fn count_on(map: &mut Vec<Vec<Vec<bool>>>) -> i64 {
    let mut retval: i64 = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            for k in 0..map[0][0].len() {
                if map[i][j][k] {
                    retval += 1;
                }
            }
        }
    }
    retval
}

#[derive(Copy, Clone)]
struct Cube {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}
impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "x[{},{}] y[{},{}] z[{},{}]",
            self.x1, self.x2, self.y1, self.y2, self.z1, self.z2
        )
    }
}

fn process_cube(line: &str, cubes: &mut Vec<Cube>) {
    // Parse lines like this: on x=-49..1,y=-3..46,z=-24..28
    let mut bounds: Vec<i32> = Vec::new();
    let on = line.chars().nth(1).unwrap() == 'n';
    let tmp: Vec<String> = line.split(' ').map(str::to_string).collect();
    let xyz: Vec<String> = tmp[1].split(',').map(str::to_string).collect();
    for bound in xyz.iter().take(3) {
        let range: Vec<String> = bound.split('=').map(str::to_string).collect();
        let nums: Vec<i32> = range[1].split("..").map(|s| s.parse().unwrap()).collect();
        bounds.push(nums[0]);
        bounds.push(nums[1]);
    }

    // Create the cube with x1 < x2, etc
    let c = Cube {
        x1: std::cmp::min(bounds[0], bounds[1]),
        x2: std::cmp::max(bounds[0], bounds[1]),
        y1: std::cmp::min(bounds[2], bounds[3]),
        y2: std::cmp::max(bounds[2], bounds[3]),
        z1: std::cmp::min(bounds[4], bounds[5]),
        z2: std::cmp::max(bounds[4], bounds[5]),
    };

    if on {
        add_cube(c, cubes);
    } else {
        remove_cube(&c, cubes);
    }
}

fn add_cube(c: Cube, cubes: &mut Vec<Cube>) {
    let mut cubes_to_insert = vec![c];
    while !cubes_to_insert.is_empty() {
        let try_cube = cubes_to_insert.pop().unwrap();
        let mut i = 0;
        while i < cubes.len() {
            if cube_intersects(&try_cube, &cubes[i]) {
                println!("{} intersects {}", try_cube, cubes[i]);
                let mut new_cubes = vec![c];
                split_cubes(&cubes[i], &mut new_cubes);
                while !new_cubes.is_empty() {
                    cubes_to_insert.push(new_cubes.pop().unwrap());
                }
                break;
            }
            println!("No collision");
            i += 1;
        }
        cubes.push(c);
    }
}
fn remove_cube(_c: &Cube, _cubes: &mut Vec<Cube>) {}

/// Returns `true` if the cubes overlap
fn cube_intersects(c1: &Cube, c2: &Cube) -> bool {
    // If both points are either less than the low point or higher than the high point,
    // the it does not intersect
    // Need all three axes to intersect to have the cube intersect
    !(c1.x1 < c2.x1 && c1.x2 < c2.x1
        || c1.x1 > c2.x2 && c1.x2 > c2.x2
        || c1.y1 < c2.y1 && c1.y2 < c2.y1
        || c1.y1 > c2.y2 && c1.y2 > c2.y2
        || c1.z1 < c2.z1 && c1.z2 < c2.z1
        || c1.z1 > c2.z2 && c1.z2 > c2.z2)
}

/// Takes 2 intersecting cubes and returns a list of split cubes
/// The `preserve_cube` will not be split
/// The `split_cubes` should have one cube submitted, which will be split
/// into several non-overlapping cubes
fn split_cubes(preserve_cube: &Cube, split_cubes: &mut Vec<Cube>) {
    let c = split_cubes.pop().unwrap();
    let mut x_segments: Vec<Option<(i32, i32)>> = vec![None; 3];
    let mut y_segments: Vec<Option<(i32, i32)>> = vec![None; 3];
    let mut z_segments: Vec<Option<(i32, i32)>> = vec![None; 3];

    if c.x1 < preserve_cube.x1 {
        x_segments[0] = Some((c.x1, preserve_cube.x1 - 1));
    }
    if c.x2 > preserve_cube.x2 {
        x_segments[2] = Some((preserve_cube.x2 + 1, c.x2));
    }
    x_segments[1] = Some((
        std::cmp::max(c.x1, preserve_cube.x1),
        std::cmp::min(c.x2, preserve_cube.x2),
    ));

    if c.y1 < preserve_cube.y1 {
        y_segments[0] = Some((c.y1, preserve_cube.y1 - 1));
    }
    if c.x2 > preserve_cube.x2 {
        y_segments[2] = Some((preserve_cube.y2 + 1, c.y2));
    }
    y_segments[1] = Some((
        std::cmp::max(c.y1, preserve_cube.y1),
        std::cmp::min(c.y2, preserve_cube.y2),
    ));

    if c.z1 < preserve_cube.z1 {
        z_segments[0] = Some((c.z1, preserve_cube.z1 - 1));
    }
    if c.z2 > preserve_cube.z2 {
        z_segments[2] = Some((preserve_cube.z2 + 1, c.z2));
    }
    z_segments[1] = Some((
        std::cmp::max(c.z1, preserve_cube.z1),
        std::cmp::min(c.z2, preserve_cube.z2),
    ));

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                if !(i == 2 && j == 2 && k == 2)
                    && (x_segments[i].is_some()
                        && y_segments[j].is_some()
                        && z_segments[k].is_some())
                {
                    split_cubes.push(Cube {
                        x1: x_segments[i].unwrap().0,
                        x2: x_segments[i].unwrap().1,
                        y1: y_segments[j].unwrap().0,
                        y2: y_segments[j].unwrap().1,
                        z1: z_segments[k].unwrap().0,
                        z2: z_segments[k].unwrap().1,
                    });
                }
            }
        }
    }
}
