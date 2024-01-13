fn bellman_ford(
    i: usize,
    n: usize,
    d: &mut Vec<i64>,
    prev: &mut Vec<usize>,
    edges: &Vec<(usize, usize, i64)>,
    visited: &mut Vec<bool>,
) -> bool {
    d[i] = 0;
    let mut end = None;
    for _ in 1..=n {
        end = None;
        for &(a, b, c) in edges {
            if d[a] != i64::MAX && d[a] + c < d[b] {
                d[b] = d[a] + c;
                prev[b] = a;
                visited[b] = true;
                // println!("update: {:?}", (i, a, b, c));
                end = Some(b);
            }
        }
    }
    if end.is_some() {
        let mut b = end.unwrap();
        for _ in 0..n {
            // println!("{} ; {:?} ", b, prev);
            b = prev[b];
        }
        let mut cycle = vec![];
        let mut node = b;
        while node != b || cycle.len() == 0 {
            cycle.push(node);
            node = prev[node];
            // println!("{} ; {} , {} ,{:?}", node, prev[b], b, prev);
        }
        cycle.push(b);
        println!("YES");
        println!(
            "{}",
            cycle
                .iter()
                .rev()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        return true;
    }

    false
}

fn dfs(node: usize, adj_list: &[Vec<(usize, i64)>], visited: &mut Vec<bool>) {
    visited[node] = true;
    for &(adj, _) in &adj_list[node] {
        if !visited[adj] {
            dfs(adj, adj_list, visited);
        }
    }
}
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let mut adj_list = vec![Vec::new(); n + 1];

    let mut edges = Vec::new();

    for _ in 0..m {
        input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        iter = input.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        let c: i64 = iter.next().unwrap().parse().unwrap();
        edges.push((a, b, c));
        adj_list[a].push((b, c))
    }
    let mut d = vec![i64::MAX; n + 1];
    let mut prev = vec![0; n + 1];
    let mut visited = vec![false; n + 1];

    for i in 1..n {
        if !visited[i] {
            d[i] = 0;
            dfs(i, &adj_list, &mut visited)
        }
    }
    if bellman_ford(1, n, &mut d, &mut prev, &edges, &mut visited) {
        return;
    }
    println!("NO")
}
