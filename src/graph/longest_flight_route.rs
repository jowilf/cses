use std::{process, vec};

const UNVISITED: i8 = 0;
const EXPLORED: i8 = 1;
const VISITED: i8 = 2;

fn dfs(node: usize, state: &mut Vec<i8>, ts: &mut Vec<usize>, adj_list: &Vec<Vec<usize>>) {
    state[node] = EXPLORED;
    for &adj in &adj_list[node] {
        if state[adj] == UNVISITED {
            dfs(adj, state, ts, adj_list);
        } else if state[adj] == EXPLORED {
            // Found cycle
            println!("IMPOSSIBLE");
            process::exit(0x0100);
        }
    }
    state[node] = VISITED;
    ts.push(node);
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nm: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = nm[0];
    let m = nm[1];

    let mut adj_list = vec![Vec::new(); n + 1];
    let mut inc_list = vec![Vec::new(); n + 1];
    let mut ts = vec![];
    let mut state = vec![UNVISITED; n + 1];

    for _ in 0..m {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let abc: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let a = abc[0];
        let b = abc[1];
        adj_list[a].push(b);
        inc_list[b].push(a);
    }

    // Topological sort
    dfs(1, &mut state, &mut ts, &adj_list);
    if state[n] == UNVISITED {
        println!("IMPOSSIBLE");
        process::exit(0x0100);
    }

    let mut parent = vec![0; n + 1];
    let mut d = vec![i64::MIN; n + 1];
    ts.reverse();

    for &node in &ts {
        if node == 1 {
            d[node] = 0;
            continue;
        }
        for &inc in &inc_list[node] {
            if d[inc] + 1 > d[node] {
                d[node] = d[inc] + 1;
                parent[node] = inc;
            }
        }
    }

    let mut path = vec![n];
    let mut node = n;
    while parent[node] != 0 {
        node = parent[node];
        path.push(node);
        // println!("node {}", node);
    }
    // path.push(1);

    // println!("{:?}", &d[1..5]);
    println!("{}", path.len());

    println!(
        "{}",
        path.iter()
            .rev()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
