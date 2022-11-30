use petgraph::algo::astar;
use petgraph::graph::{Graph, NodeIndex};

use crate::Solve;

pub struct Problem {
    data: Vec<Vec<i64>>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        algorithm3(&self.data)
    }
    fn p2(&mut self) -> i64 {
        let big = expand_data(&self.data, 4);

        algorithm3(&big)
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        let mut g: Vec<Vec<i64>> = Vec::new();

        for line in input {
            let mut tmp: Vec<i64> = Vec::new();
            for c in line.chars() {
                tmp.push(i64::from(c.to_digit(10).unwrap()));
            }
            g.push(tmp);
        }

        Problem { data: g }
    }
}

#[allow(dead_code)]
fn algorithm1(g: &[Vec<i64>]) -> i64 {
    let mut scores: Vec<i64> = Vec::new();
    let max = g.len();
    wander(g, &mut scores, 1, 0, max, 0);
    println!("===HALF WAY===");
    wander(g, &mut scores, 0, 1, max, 0);

    *scores.iter().min().unwrap()
}

#[allow(dead_code)]
fn wander(g: &[Vec<i64>], scores: &mut Vec<i64>, row: usize, col: usize, max: usize, score: i64) {
    //println!("{},{} {}", row, col, max);
    // Stop wandering if you are already over min
    if score < *scores.iter().min().unwrap_or(&i64::MAX) {
        if (row == max - 1) && (col == max - 1) {
            scores.push(score + g[row][col]);
        } else {
            if row < max - 1 {
                wander(g, scores, row + 1, col, max, score + g[row][col]);
            }
            if col < max - 1 {
                wander(g, scores, row, col + 1, max, score + g[row][col]);
            }
        }
    } else {
        // println!(
        //     ">>>>> Bailing {} > {}",
        //     score,
        //     *scores.iter().min().unwrap_or(&i64::MAX)
        // );
    }
}
// #[allow(dead_code)]
// fn algorithm2() -> i64 {
//     let mut g = Graph::<i32, i32>::new();

//     let a = g.add_node(1);
//     let b = g.add_node(1);
//     let c = g.add_node(6);
//     let d = g.add_node(1);
//     let e = g.add_node(3);
//     let f = g.add_node(8);
//     let gg = g.add_node(2);
//     let h = g.add_node(1);
//     let i = g.add_node(3);

//     g.add_edge(a, b, 1);
//     g.add_edge(b, c, 6);
//     g.add_edge(a, d, 1);
//     g.add_edge(b, e, 2);
//     g.add_edge(c, f, 8);

//     g.add_edge(d, e, 3);
//     g.add_edge(e, f, 8);
//     g.add_edge(d, gg, 2);
//     g.add_edge(e, h, 1);
//     g.add_edge(f, i, 3);

//     g.add_edge(gg, h, 1);
//     g.add_edge(h, i, 3);
//     g.add_edge(h, gg, 1);
//     println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

//     let path =
//         petgraph::algo::astar(&g, a, |finish| finish == gg, |ed| *ed.weight(), |_| 0).unwrap();

//     println!("Length: {}", path.0);
//     for x in path.1 {
//         println!("{:?}", g.node_weight(x));
//     }
//     0
// }

fn algorithm3(data: &[Vec<i64>]) -> i64 {
    let mut nodes: Vec<Vec<NodeIndex>> = Vec::new();
    let g = build_graph(data, &mut nodes);
    let end = nodes[nodes.len() - 1][nodes.len() - 1];
    let mut retval: i64 = 0;

    let path = astar(&g, nodes[0][0], |e| e == end, |ed| *ed.weight(), |_| 0).unwrap();

    for x in path.1 {
        retval += g.node_weight(x).unwrap();
    }
    retval -= g.node_weight(nodes[0][0]).unwrap();

    retval
}

fn build_graph(data: &[Vec<i64>], nodes: &mut Vec<Vec<NodeIndex>>) -> Graph<i64, i64> {
    let mut g = Graph::<i64, i64>::new();

    let len = data.len();
    for row in data {
        let mut tmp: Vec<NodeIndex> = Vec::new();
        for col in row {
            tmp.push(g.add_node(*col));
        }
        nodes.push(tmp);
    }

    // Setup directed edges into node
    for row in 0..len {
        for col in 0..len {
            let w = *g.node_weight(nodes[row][col]).unwrap();
            if row > 0 {
                g.add_edge(nodes[row - 1][col], nodes[row][col], w);
            }
            if col < len - 1 {
                g.add_edge(nodes[row][col + 1], nodes[row][col], w);
            }
            if row < len - 1 {
                g.add_edge(nodes[row + 1][col], nodes[row][col], w);
            }
            if col > 0 {
                g.add_edge(nodes[row][col - 1], nodes[row][col], w);
            }
        }
    }

    g
}

fn expand_data(data: &[Vec<i64>], grow: usize) -> Vec<Vec<i64>> {
    let mut big: Vec<Vec<i64>> = data.to_vec();

    for _i in 0..grow {
        expand_right(&mut big, data.len());
    }
    for _i in 0..grow {
        expand_down(&mut big, data.len());
    }

    big
}

fn expand_right(data: &mut Vec<Vec<i64>>, cols: usize) {
    let start: usize = data[0].len() - cols;
    for row in data {
        for col in 0..cols {
            let mut val = row[start + col] + 1;
            if val > 9 {
                val = 1;
            }
            row.push(val);
        }
    }
}

fn expand_down(data: &mut Vec<Vec<i64>>, rows: usize) {
    let start = data.len() - rows;
    for row in 0..rows {
        let mut tmp: Vec<i64> = Vec::new();
        for col in 0..data[0].len() {
            let mut val = data[start + row][col] + 1;
            if val > 9 {
                val = 1;
            }
            tmp.push(val);
        }
        data.push(tmp);
    }
}
