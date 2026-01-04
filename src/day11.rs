use ndarray::{linalg, prelude::*};
use std::{collections::HashMap, mem::swap};

type KT = usize;
type Graph = HashMap<KT, Vec<KT>>;

fn make_keys(contents: &String) -> Option<HashMap<&str, usize>> {
    let mut keys = HashMap::from_iter(contents.lines().enumerate().map(|(idx, s)| (&s[..3], idx)));
    keys.insert("out", keys.len());
    return Some(keys);
}

fn create_adjacency(graph: &Graph) -> Option<Array2<u64>> {
    let n_nodes = graph.len();
    let mut adj: Array2<u64> = Array2::default((n_nodes, n_nodes));
    for (&k, v) in graph.iter() {
        for &idx in v {
            adj[[k, idx]] = 1;
        }
    }
    return Some(adj);
}

fn create_key_value(keys: &HashMap<&str, usize>, line: &str) -> Option<(KT, Vec<KT>)> {
    let mut sp = line.split_whitespace();
    let key = *keys.get(&sp.next().unwrap()[..3])?;
    let val: Vec<_> = sp.map(|s| *keys.get(s).unwrap()).collect();
    return Some((key, val));
}

fn make_graph(contents: &String) -> Option<(HashMap<&str, usize>, Graph)> {
    let keys = make_keys(contents)?;
    let mut graph = Graph::from_iter(
        contents
            .lines()
            .map(|line| create_key_value(&keys, line).unwrap()),
    );
    graph.insert(keys.len() - 1, Vec::new());
    return Some((keys, graph));
}

/// Returns number of paths from curr_node to target_node
fn depth_first_helper(
    graph: &Graph,
    curr_node: KT,
    visited: &mut Vec<bool>,
    target_node: KT,
) -> Option<u64> {
    if curr_node == target_node {
        return Some(1);
    }
    visited[curr_node] = true;
    let neighbors = graph.get(&curr_node)?;
    let mut ways = 0;
    for &neighbor in neighbors {
        if !visited[neighbor] {
            let way = depth_first_helper(graph, neighbor, visited, target_node);
            println!("{}, {}, {}", curr_node, neighbor, target_node);
            ways += way.unwrap();
        }
    }
    visited[curr_node] = false;
    return Some(ways);
}

fn part1(keys: &HashMap<&str, usize>, graph: &Graph) -> Option<u64> {
    let node0 = *keys.get("you")?;
    let node1 = *keys.get("out")?;
    let mut visited = (0..keys.len()).map(|_| false).collect();
    let ways1 = depth_first_helper(graph, node0, &mut visited, node1)?;
    return Some(ways1);
}

fn part2(keys: &HashMap<&str, usize>, graph: &Graph) -> Option<u64> {
    let node0 = *keys.get("svr")?;
    let node1 = *keys.get("dac")?;
    let node2 = *keys.get("fft")?;
    let node3 = *keys.get("out")?;
    let mut visited = (0..keys.len()).map(|_| false).collect();
    let ways1 = depth_first_helper(graph, node0, &mut visited, node1)?;
    println!("{}", ways1);
    visited.fill(false);
    let ways2 = depth_first_helper(graph, node1, &mut visited, node2)?;
    println!("{}", ways2);
    visited.fill(false);
    let ways3 = depth_first_helper(graph, node2, &mut visited, node3)?;
    println!("{}", ways3);
    return Some(ways1 * ways2 * ways3);
}

fn distance_linalg(adj: ArrayView2<u64>, start: usize, end: usize) -> Option<u64> {
    let mut v_old = Array1::from_iter((0..adj.shape()[0]).map(|_| 0));
    let mut v_new = v_old;
    loop {
        v_old = v_new;
        v_new = adj.dot(&v_old);
        v_new[[end]] = 1;
        if v_old == v_new {
            break;
        }
    }
    return Some(v_old[start]);
}

fn part2_linalg(keys: &HashMap<&str, usize>, graph: &Graph) -> Option<u64> {
    let adj = create_adjacency(graph)?;
    let svr = *keys.get("svr")?;
    let dac = *keys.get("dac")?;
    let fft = *keys.get("fft")?;
    let out = *keys.get("out")?;
    let svr_dac = distance_linalg(adj.view(), svr, dac)?;
    let dac_fft = distance_linalg(adj.view(), dac, fft)?;
    let fft_out = distance_linalg(adj.view(), fft, out)?;
    let svr_fft = distance_linalg(adj.view(), svr, fft)?;
    let fft_dac = distance_linalg(adj.view(), fft, dac)?;
    let dac_out = distance_linalg(adj.view(), dac, out)?;
    return Some(svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out);
}

pub fn day11(contents: &String) {
    let (keys, graph) = make_graph(contents).unwrap();
    let p1 = part1(&keys, &graph).unwrap();
    println!("Part 1: {:?}", p1);
    let p2 = part2_linalg(&keys, &graph).unwrap();
    println!("Part 2: {:?}", p2);
}
