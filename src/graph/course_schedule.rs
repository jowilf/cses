use std::process;

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
    let mut inc_cnt = vec![0; n + 1];
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
        inc_cnt[b] += 1;
    }

    for i in 1..=n {
        if state[i] == UNVISITED && inc_cnt[i] == 0 {
            dfs(i, &mut state, &mut ts, &adj_list);
        }
    }
    for i in 1..=n {
        if state[i] == UNVISITED {
            println!("IMPOSSIBLE");
            process::exit(0x0100);
        }
    }
    println!(
        "{}",
        ts.iter()
            .rev()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
