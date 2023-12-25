use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        // assumes that all edges that need to be cut have unique nodes; no node has two cut edges
        let edges = parse_input(&input);
        let total_count = edges.len();
        for (ei, e) in edges.iter().enumerate() {
            log::debug!("{ei} {e:?}");
        }
        log::info!("{total_count} nodes");

        let mut values: HashMap<(usize, usize), u64> = HashMap::new();
        for r in 0..(edges.len() / 2) {
            let root = r * 2;
            log::info!("{root}");
            let tree = build_spanning_tree(&edges, root);
            let counts = count_children(&tree);
            log::debug!("{counts:?}");
            for i in 0..tree.len() {
                let (a, b) = tree[i];
                let normalized = if a > b { (b, a) } else { (a, b) };
                *values.entry(normalized).or_default() += counts[i];
            }
        }
        let mut sorted = values.into_iter().collect_vec();
        sorted.sort_by_key(|it| it.1);
        log::debug!("{sorted:#?}");
        log::info!("{:#?}", &sorted[sorted.len() - 3..]);

        let mut edges = edges;
        for ((a, b), _) in &sorted[sorted.len() - 3..] {
            edges[*a].retain(|x| x != b);
            edges[*b].retain(|x| x != a);
        }

        let count = count_connected(&edges, 0);
        log::info!("{count}");
        (count * (total_count - count)).to_string()
    }

    fn solve_2(&self, _input: String) -> String {
        "Merry Christmas!".to_string()
    }
}

fn parse_input<'a>(input: &'a str) -> Vec<Vec<usize>> {
    let mut nodes: HashMap<&str, usize> = HashMap::new();
    let mut adj = Vec::new();

    let mut get_node_i = |name: &'a str, adj: &mut Vec<Vec<usize>>| {
        if let Some(i) = nodes.get(&name) {
            *i
        } else {
            let i = nodes.len();
            nodes.insert(name, i);
            adj.push(Vec::new());
            i
        }
    };

    for line in input.lines() {
        let (name, edges_str) = line.split_once(": ").unwrap();
        let a = get_node_i(name, &mut adj);
        for dest in edges_str.split_ascii_whitespace() {
            let b = get_node_i(dest, &mut adj);
            adj[a].push(b);
            adj[b].push(a);
        }
    }

    log::debug!("{nodes:?}");
    adj
}

fn build_spanning_tree(graph: &[Vec<usize>], root: usize) -> Vec<(usize, usize)> {
    let mut q = VecDeque::new();
    q.push_back(root);
    let mut visited = HashSet::new();
    visited.insert(root);
    let mut res = vec![];
    while let Some(n) = q.pop_front() {
        let next = graph[n]
            .iter()
            .copied()
            .filter(|neigh| !visited.contains(neigh))
            .collect_vec();
        for m in next {
            res.push((n, m));
            q.push_back(m);
            visited.insert(m);
        }
    }
    res
}

fn count_children(tree: &[(usize, usize)]) -> Vec<u64> {
    let mut res = vec![0; tree.len()];
    for i in (0..tree.len()).rev() {
        let (_, m) = tree[i];
        res[i] += tree
            .iter()
            .enumerate()
            .dropping(i)
            .map(|(ei, e)| if e.0 == m { res[ei] + 1 } else { 0 })
            .sum::<u64>();
    }
    res
}

fn count_connected(graph: &[Vec<usize>], start: usize) -> usize {
    let mut q = vec![start];
    let mut visited = HashSet::new();
    while let Some(n) = q.pop() {
        log::debug!("visiting {n}");
        if visited.insert(n) {
            q.extend(graph[n].iter().copied());
        }
    }
    visited.len()
}
