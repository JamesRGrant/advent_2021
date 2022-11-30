use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;
use std::collections::HashSet;

use crate::Solve;

pub struct Problem {
    s: Vec<Vec<Point>>,
    nodes: Vec<NodeIndex>,
    g: Graph<usize, ()>,
    offsets: HashMap<(usize, usize), Point>,
    xform: HashMap<(usize, usize), String>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        let mut final_nodes: HashSet<Point> = HashSet::new();
        for p in &self.s[0] {
            final_nodes.insert(*p);
        }
        for i in 1..self.s.len() {
            let path = petgraph::algo::astar(
                &self.g,
                self.nodes[i],
                |finish| finish == self.nodes[0],
                |_| 0,
                |_| 0,
            )
            .unwrap();

            let mut points = self.s[i].clone();

            for step in 0..(path.1.len() - 1) {
                let from: usize = *self.g.node_weight(path.1[step]).unwrap();
                let to: usize = *self.g.node_weight(path.1[step + 1]).unwrap();

                if self.offsets.contains_key(&(from, to)) {
                    let diff = self.offsets.get(&(from, to)).unwrap();
                    let new_diff = reverse_diff(self.xform.get(&(from, to)).unwrap(), *diff);

                    xform_points(
                        &mut points,
                        new_diff,
                        &reverse_axis(self.xform.get(&(from, to)).unwrap()),
                        false,
                    );
                } else if self.offsets.contains_key(&(to, from)) {
                    xform_points(
                        &mut points,
                        *self.offsets.get(&(to, from)).unwrap(),
                        self.xform.get(&(to, from)).unwrap(),
                        false,
                    );
                } else {
                    panic!("It's a disco!");
                }
            }
            for p in points {
                final_nodes.insert(p);
            }
        }
        final_nodes.len() as i64
    }
    fn p2(&mut self) -> i64 {
        let mut max = 0;
        for i in 0..self.s.len() {
            for j in 0..self.s.len() {
                if i != j {
                    let path = petgraph::algo::astar(
                        &self.g,
                        self.nodes[i],
                        |finish| finish == self.nodes[0],
                        |_| 0,
                        |_| 0,
                    )
                    .unwrap();
                    let mut points: Vec<Point> = vec![Point { x: 0, y: 0, z: 0 }];

                    for step in 0..(path.1.len() - 1) {
                        let from: usize = *self.g.node_weight(path.1[step]).unwrap();
                        let to: usize = *self.g.node_weight(path.1[step + 1]).unwrap();

                        if self.offsets.contains_key(&(from, to)) {
                            let diff = self.offsets.get(&(from, to)).unwrap();
                            let new_diff =
                                reverse_diff(self.xform.get(&(from, to)).unwrap(), *diff);

                            xform_points(
                                &mut points,
                                new_diff,
                                &reverse_axis(self.xform.get(&(from, to)).unwrap()),
                                false,
                            );
                        } else if self.offsets.contains_key(&(to, from)) {
                            xform_points(
                                &mut points,
                                *self.offsets.get(&(to, from)).unwrap(),
                                self.xform.get(&(to, from)).unwrap(),
                                false,
                            );
                        } else {
                            panic!("It's a disco!");
                        }
                    }

                    let this =
                        i16::abs(points[0].x) + i16::abs(points[0].y) + i16::abs(points[0].z);
                    max = std::cmp::max(max, this);
                }
            }
        }

        i64::from(max)
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        let s = load_scanners(input);
        let mut nodes: Vec<NodeIndex> = Vec::new();
        let mut g = load_nodes(s.len(), &mut nodes);
        let mut offsets: HashMap<(usize, usize), Point> = HashMap::new();
        let mut xform: HashMap<(usize, usize), String> = HashMap::new();
        load_edges(&s, &mut g, &nodes, &mut offsets, &mut xform);
        Problem {
            s,
            nodes,
            g,
            offsets,
            xform,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Copy, Debug)]
struct Point {
    x: i16,
    y: i16,
    z: i16,
}

fn load_scanners(v: &[String]) -> Vec<Vec<Point>> {
    let mut scanners: Vec<Vec<Point>> = Vec::new();
    let mut s = 0;
    scanners.push(Vec::new());

    for line in v {
        if line.is_empty() {
            scanners.push(Vec::new());
            s += 1;
        } else if &line[0..3] == "---" {
        } else {
            let nums: Vec<i16> = line.split(',').map(|s| s.parse().unwrap()).collect();
            scanners[s].push(Point {
                x: nums[0],
                y: nums[1],
                z: nums[2],
            });
        }
    }
    scanners
}

fn load_nodes(size: usize, nodes: &mut Vec<NodeIndex>) -> Graph<usize, ()> {
    let mut g = Graph::<usize, ()>::new();
    for i in 0..size {
        let tmp = g.add_node(i);
        nodes.push(tmp);
    }
    g
}

fn load_edges(
    s: &[Vec<Point>],
    g: &mut Graph<usize, ()>,
    nodes: &[NodeIndex],
    offsets: &mut HashMap<(usize, usize), Point>,
    xform: &mut HashMap<(usize, usize), String>,
) {
    for i in 0..s.len() {
        for j in (i + 1)..s.len() {
            let (diff, axis) = match_beacons(&s[i], &s[j]);
            if diff.x != 0 && diff.y != 0 && diff.z != 0 {
                g.add_edge(nodes[i], nodes[j], ());
                g.add_edge(nodes[j], nodes[i], ());
                offsets.insert(
                    (i, j),
                    Point {
                        x: diff.x,
                        y: diff.y,
                        z: diff.z,
                    },
                );
                xform.insert((i, j), axis.clone());
            }
        }
    }
}

fn match_beacons(a: &[Point], b: &[Point]) -> (Point, String) {
    let mut retval = Point { x: 0, y: 0, z: 0 };
    let mut pairs: Vec<(&Point, &Point)> = Vec::new();

    for i in 0..a.len() {
        for j in 0..b.len() {
            let mut matches = 0;
            for k in 0..a.len() {
                if i != k {
                    let len_a = len_3d(a[i], a[k]);
                    for l in 0..b.len() {
                        if j != l {
                            let len_b = len_3d(b[j], b[l]);
                            // len_a == len_b, safe compare for precision
                            if (len_a - len_b).abs() < f64::EPSILON {
                                matches += 1;
                            }
                        }
                    }
                }
            }

            if matches >= 11 {
                pairs.push((&a[i], &b[j]));
            }
        }
    }
    let mut axis = String::new();
    if pairs.len() >= 12 {
        retval = find_xform(&pairs, &mut axis);
    }
    (retval, axis)
}

fn len_3d(a: Point, b: Point) -> f64 {
    f64::sqrt(
        f64::from(b.x - a.x) * f64::from(b.x - a.x)
            + f64::from(b.y - a.y) * f64::from(b.y - a.y)
            + f64::from(b.z - a.z) * f64::from(b.z - a.z),
    )
}

fn find_xform(pairs: &[(&Point, &Point)], axis: &mut String) -> Point {
    let x: i16 = xform_x(pairs, axis);
    let mut y: i16 = 0;
    let mut z: i16 = 0;

    if pairs[0].0.y - pairs[0].1.x == pairs[1].0.y - pairs[1].1.x
        && pairs[0].0.y - pairs[0].1.x == pairs[2].0.y - pairs[2].1.x
    {
        y = pairs[0].0.y - pairs[0].1.x;
        axis.push_str("+x");
    }
    if pairs[0].0.y + pairs[0].1.x == pairs[1].0.y + pairs[1].1.x
        && pairs[0].0.y + pairs[0].1.x == pairs[2].0.y + pairs[2].1.x
    {
        y = pairs[0].0.y + pairs[0].1.x;
        axis.push_str("-x");
    }
    if pairs[0].0.y - pairs[0].1.y == pairs[1].0.y - pairs[1].1.y
        && pairs[0].0.y - pairs[0].1.y == pairs[2].0.y - pairs[2].1.y
    {
        y = pairs[0].0.y - pairs[0].1.y;
        axis.push_str("+y");
    }
    if pairs[0].0.y + pairs[0].1.y == pairs[1].0.y + pairs[1].1.y
        && pairs[0].0.y + pairs[0].1.y == pairs[2].0.y + pairs[2].1.y
    {
        y = pairs[0].0.y + pairs[0].1.y;
        axis.push_str("-y");
    }
    if pairs[0].0.y - pairs[0].1.z == pairs[1].0.y - pairs[1].1.z
        && pairs[0].0.y - pairs[0].1.z == pairs[2].0.y - pairs[2].1.z
    {
        y = pairs[0].0.y - pairs[0].1.z;
        axis.push_str("+z");
    }
    if pairs[0].0.y + pairs[0].1.z == pairs[1].0.y + pairs[1].1.z
        && pairs[0].0.y + pairs[0].1.z == pairs[2].0.y + pairs[2].1.z
    {
        y = pairs[0].0.y + pairs[0].1.z;
        axis.push_str("-z");
    }

    if pairs[0].0.z - pairs[0].1.x == pairs[1].0.z - pairs[1].1.x
        && pairs[0].0.z - pairs[0].1.x == pairs[2].0.z - pairs[2].1.x
    {
        z = pairs[0].0.z - pairs[0].1.x;
        axis.push_str("+x");
    }
    if pairs[0].0.z + pairs[0].1.x == pairs[1].0.z + pairs[1].1.x
        && pairs[0].0.z + pairs[0].1.x == pairs[2].0.z + pairs[2].1.x
    {
        z = pairs[0].0.z + pairs[0].1.x;
        axis.push_str("-x");
    }
    if pairs[0].0.z - pairs[0].1.y == pairs[1].0.z - pairs[1].1.y
        && pairs[0].0.z - pairs[0].1.y == pairs[2].0.z - pairs[2].1.y
    {
        z = pairs[0].0.z - pairs[0].1.y;
        axis.push_str("+y");
    }
    if pairs[0].0.z + pairs[0].1.y == pairs[1].0.z + pairs[1].1.y
        && pairs[0].0.z + pairs[0].1.y == pairs[2].0.z + pairs[2].1.y
    {
        z = pairs[0].0.z + pairs[0].1.y;
        axis.push_str("-y");
    }
    if pairs[0].0.z - pairs[0].1.z == pairs[1].0.z - pairs[1].1.z
        && pairs[0].0.z - pairs[0].1.z == pairs[2].0.z - pairs[2].1.z
    {
        z = pairs[0].0.z - pairs[0].1.z;
        axis.push_str("+z");
    }
    if pairs[0].0.z + pairs[0].1.z == pairs[1].0.z + pairs[1].1.z
        && pairs[0].0.z + pairs[0].1.z == pairs[2].0.z + pairs[2].1.z
    {
        z = pairs[0].0.z + pairs[0].1.z;
        axis.push_str("-z");
    }

    if axis.len() != 6 {
        for p in pairs {
            println!(
                "{},{},{},,{},{},{}",
                p.0.x, p.0.y, p.0.z, p.1.x, p.1.y, p.1.z
            );
        }
        println!("{axis}");
        panic!("OH NO!!!!!!!!!!!!!!!!!!!!!!");
    }

    Point { x, y, z }
}

fn xform_x(pairs: &[(&Point, &Point)], axis: &mut String) -> i16 {
    let mut x: i16 = 0;

    if pairs[0].0.x - pairs[0].1.x == pairs[1].0.x - pairs[1].1.x
        && pairs[0].0.x - pairs[0].1.x == pairs[2].0.x - pairs[2].1.x
    {
        x = pairs[0].0.x - pairs[0].1.x;
        axis.push_str("+x");
    }
    if pairs[0].0.x + pairs[0].1.x == pairs[1].0.x + pairs[1].1.x
        && pairs[0].0.x + pairs[0].1.x == pairs[2].0.x + pairs[2].1.x
    {
        x = pairs[0].0.x + pairs[0].1.x;
        axis.push_str("-x");
    }
    if pairs[0].0.x - pairs[0].1.y == pairs[1].0.x - pairs[1].1.y
        && pairs[0].0.x - pairs[0].1.y == pairs[2].0.x - pairs[2].1.y
    {
        x = pairs[0].0.x - pairs[0].1.y;
        axis.push_str("+y");
    }
    if pairs[0].0.x + pairs[0].1.y == pairs[1].0.x + pairs[1].1.y
        && pairs[0].0.x + pairs[0].1.y == pairs[2].0.x + pairs[2].1.y
    {
        x = pairs[0].0.x + pairs[0].1.y;
        axis.push_str("-y");
    }
    if pairs[0].0.x - pairs[0].1.z == pairs[1].0.x - pairs[1].1.z
        && pairs[0].0.x - pairs[0].1.z == pairs[2].0.x - pairs[2].1.z
    {
        x = pairs[0].0.x - pairs[0].1.z;
        axis.push_str("+z");
    }
    if pairs[0].0.x + pairs[0].1.z == pairs[1].0.x + pairs[1].1.z
        && pairs[0].0.x + pairs[0].1.z == pairs[2].0.x + pairs[2].1.z
    {
        x = pairs[0].0.x + pairs[0].1.z;
        axis.push_str("-z");
    }
    x
}

fn xform_points(points: &mut Vec<Point>, diff: Point, xform: &str, print: bool) {
    let tmp: Vec<Point> = points.drain(..).collect();
    for point in tmp {
        let x = match &xform[0..2] {
            "+x" => point.x + diff.x,
            "-x" => -(point.x - diff.x),
            "+y" => point.y + diff.x,
            "-y" => -(point.y - diff.x),
            "+z" => point.z + diff.x,
            "-z" => -(point.z - diff.x),
            _ => panic!("Bad x xform"),
        };
        let y = match &xform[2..4] {
            "+x" => point.x + diff.y,
            "-x" => -(point.x - diff.y),
            "+y" => point.y + diff.y,
            "-y" => -(point.y - diff.y),
            "+z" => point.z + diff.y,
            "-z" => -(point.z - diff.y),
            _ => panic!("Bad y xform"),
        };
        let z = match &xform[4..6] {
            "+x" => point.x + diff.z,
            "-x" => -(point.x - diff.z),
            "+y" => point.y + diff.z,
            "-y" => -(point.y - diff.z),
            "+z" => point.z + diff.z,
            "-z" => -(point.z - diff.z),
            _ => panic!("Bad z xform"),
        };
        if print {
            println!(
                " ({},{},{}) -> ({},{},{}) with ({},{},{})",
                point.x, point.y, point.z, x, y, z, diff.x, diff.y, diff.z
            );
        }

        points.push(Point { x, y, z });
    }
}

fn reverse_diff(xform: &str, diff: Point) -> Point {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    match &xform[0..2] {
        "+x" => x = -diff.x,
        "-x" => x = diff.x,
        "+y" => y = -diff.x,
        "-y" => y = diff.x,
        "+z" => z = -diff.x,
        "-z" => z = diff.x,
        _ => panic!("Bad x xform"),
    }
    match &xform[2..4] {
        "+x" => x = -diff.y,
        "-x" => x = diff.y,
        "+y" => y = -diff.y,
        "-y" => y = diff.y,
        "+z" => z = -diff.y,
        "-z" => z = diff.y,
        _ => panic!("Bad y xform"),
    }
    match &xform[4..6] {
        "+x" => x = -diff.z,
        "-x" => x = diff.z,
        "+y" => y = -diff.z,
        "-y" => y = diff.z,
        "+z" => z = -diff.z,
        "-z" => z = diff.z,
        _ => panic!("Bad z xform"),
    }
    Point { x, y, z }
}

fn reverse_axis(xform: &str) -> String {
    let mut s = String::new();
    let mut x = "";
    let mut y = "";
    let mut z = "";
    match &xform[0..2] {
        "+x" => x = "+x",
        "-x" => x = "-x",
        "+y" => y = "+x",
        "-y" => y = "-x",
        "+z" => z = "+x",
        "-z" => z = "-x",
        _ => panic!("Bad x xform"),
    }
    match &xform[2..4] {
        "+x" => x = "+y",
        "-x" => x = "-y",
        "+y" => y = "+y",
        "-y" => y = "-y",
        "+z" => z = "+y",
        "-z" => z = "-y",
        _ => panic!("Bad y xform"),
    }
    match &xform[4..6] {
        "+x" => x = "+z",
        "-x" => x = "-z",
        "+y" => y = "+z",
        "-y" => y = "-z",
        "+z" => z = "+z",
        "-z" => z = "-z",
        _ => panic!("Bad z xform"),
    }
    s.push_str(x);
    s.push_str(y);
    s.push_str(z);
    s
}
