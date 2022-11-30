use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;

use crate::Solve;

pub struct Problem {
    nodes: HashMap<String, NodeIndex>,
    g: UnGraph<String, ()>,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        find_paths(&self.g, self.nodes["start"], "".to_string())
    }
    fn p2(&mut self) -> i64 {
        find_paths2(&self.g, self.nodes["start"], "".to_string(), false)
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
        let g = load_graph(input, &mut nodes);
        Problem { nodes, g }
    }
}

fn load_graph(v: &[String], nodes: &mut HashMap<String, NodeIndex>) -> UnGraph<String, ()> {
    let mut g = UnGraph::<String, ()>::new_undirected();
    for line in v {
        let tmp: Vec<String> = line.split('-').map(str::to_string).collect();
        if !nodes.contains_key(&tmp[0]) {
            nodes.insert(tmp[0].clone(), g.add_node(tmp[0].clone()));
        }
        if !nodes.contains_key(&tmp[1]) {
            nodes.insert(tmp[1].clone(), g.add_node(tmp[1].clone()));
        }
        g.add_edge(nodes[&tmp[0]], nodes[&tmp[1]], ());
    }
    // println!("{:?}", petgraph::dot::Dot::new(&g));
    g
}

fn find_paths(g: &UnGraph<String, ()>, node_index: NodeIndex, path: String) -> i64 {
    let mut new_path = path;
    let mut retval = 0;

    if &g[node_index] == "start" {
        new_path.push_str(&g[node_index]);
        new_path.push(',');
    } else if &g[node_index] == "end" {
        return 1;
    }

    for n in g.neighbors(node_index) {
        let mut search: String = g[n].to_string();
        search.push(',');

        if !new_path.contains(search.as_str()) || g[n].chars().next().unwrap().is_uppercase() {
            let mut new_new_path: String = new_path.clone();
            new_new_path.push_str(&g[n]);
            new_new_path.push(',');
            retval += find_paths(g, n, new_new_path);
        }
    }
    retval
}

fn find_paths2(
    g: &UnGraph<String, ()>,
    node_index: NodeIndex,
    path: String,
    second_small: bool,
) -> i64 {
    let mut new_path = path;
    let nav_children;
    let mut retval = 0;

    if &g[node_index] == "start" {
        if new_path.is_empty() {
            new_path.push_str(&g[node_index]);
            new_path.push(',');
            nav_children = true;
        } else {
            nav_children = false;
        }
    } else if &g[node_index] == "end" {
        return 1;
    } else {
        nav_children = true;
    }

    if nav_children {
        for n in g.neighbors(node_index) {
            let mut search: String = g[n].to_string();
            search.push(',');
            let mut new_new_path: String = new_path.clone();

            if !new_path.contains(search.as_str()) || g[n].chars().next().unwrap().is_uppercase() {
                new_new_path.push_str(&g[n]);
                new_new_path.push(',');
                retval += find_paths2(g, n, new_new_path, second_small);
            } else if !second_small {
                new_new_path.push_str(&g[n]);
                new_new_path.push(',');
                retval += find_paths2(g, n, new_new_path, true);
            }
        }
    }

    retval
}
