use std::collections::VecDeque;

const UNVISITED: i8 = 0;
const EXPLORED: i8 = 1;
const VISITED: i8 = 2;

fn dfs1(node: usize, state: &mut Vec<i8>, adj_list: &Vec<Vec<usize>>, stack: &mut VecDeque<usize>) {
    state[node] = EXPLORED;
    for &adj in &adj_list[node] {
        if state[adj] == UNVISITED {
            dfs1(adj, state, adj_list, stack);
        }
    }
    state[node] = VISITED;
    stack.push_front(node);
}

fn dfs2(
    node: usize,
    state: &mut Vec<i8>,
    adj_list: &Vec<Vec<usize>>,
    ssc: &mut Vec<i32>,
    cnt: i32,
) {
    state[node] = EXPLORED;
    for &adj in &adj_list[node] {
        if state[adj] == UNVISITED {
            dfs2(adj, state, adj_list, ssc, cnt);
        }
    }
    state[node] = VISITED;
    ssc[node] = cnt;
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
    let mut rev_list = vec![Vec::new(); n + 1];
    let mut state = vec![UNVISITED; n + 1];
    let mut stack = VecDeque::new();
    let mut scc = vec![1; n + 1];

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
        rev_list[b].push(a);
    }
    for i in 1..=n {
        if state[i] == UNVISITED {
            dfs1(i, &mut state, &adj_list, &mut stack);
        }
    }
    state = vec![UNVISITED; n + 1];
    let mut cnt = 0;
    // println!("stack: {:?}", stack);
    while !stack.is_empty() {
        let node = stack.pop_front().unwrap();
        if state[node] == UNVISITED {
            cnt += 1;
            // println!("dfs", )
            dfs2(node, &mut state, &rev_list, &mut scc, cnt);
        }
    }
    println!("{}", cnt);
    println!(
        "{}",
        &scc[1..]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}