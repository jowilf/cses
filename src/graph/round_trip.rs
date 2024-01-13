use std::process;

const UNVISITED: i8 = 0;
const EXPLORED: i8 = 1;
const VISITED: i8 = 2;

fn dfs(node: usize, state: &mut Vec<i8>, parent: &mut Vec<usize>, adj_list: &Vec<Vec<usize>>) {
    state[node] = EXPLORED;
    for &adj in &adj_list[node] {
        if state[adj] == UNVISITED {
            parent[adj] = node;
            dfs(adj, state, parent, adj_list);
        } else if state[adj] == EXPLORED && parent[node] != adj {
            // Found cycle
            let mut path = vec![adj, node];
            let mut n = node;
            while parent[n] != adj {
                n = parent[n];
                path.push(n);
            }
            path.push(adj);
            println!("{}", path.len());
            println!(
                "{}",
                path.iter()
                    .rev()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
            process::exit(0x0100);
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
    let mut parent = vec![0; n + 1];
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
        adj_list[b].push(a);
    }

    for i in 1..=n {
        if state[i] == UNVISITED {
            dfs(i, &mut state, &mut parent, &adj_list);
        }
    }
    println!("IMPOSSIBLE")
}
