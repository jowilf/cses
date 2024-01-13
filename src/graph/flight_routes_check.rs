const UNVISITED: i8 = 0;
const EXPLORED: i8 = 1;
const VISITED: i8 = 2;

fn dfs(node: usize, state: &mut Vec<i8>, adj_list: &Vec<Vec<usize>>) {
    state[node] = EXPLORED;
    for &adj in &adj_list[node] {
        if state[adj] == UNVISITED {
            dfs(adj, state, adj_list);
        }
    }
    state[node] = VISITED;
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
    dfs(1, &mut state, &adj_list);
    for i in 1..=n {
        if state[i] == UNVISITED {
            println!("NO");
            println!("{} {}", 1, i);
            return;
        }
        state[i] = UNVISITED;
    }
    dfs(1, &mut state, &rev_list);
    for i in 1..=n {
        if state[i] == UNVISITED {
            println!("NO");
            println!("{} {}", i, 1);
            return;
        }
    }
    println!("YES")
}
