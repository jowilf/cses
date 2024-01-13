fn dfs(node: usize, rev_adj_list: &Vec<Vec<(usize, i64)>>, visited: &mut Vec<bool>) {
    visited[node] = true;
    for &(adj, _) in &rev_adj_list[node] {
        if !visited[adj] {
            dfs(adj, rev_adj_list, visited);
        }
    }
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

    let mut rev_adj_list = vec![Vec::new(); n + 1];
    let mut visited = vec![false; n + 1];
    let mut edges = Vec::new();
    let mut d = vec![i64::MIN; n + 1];

    for _ in 0..m {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let abc: Vec<i64> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let a = abc[0] as usize;
        let b = abc[1] as usize;
        let c = abc[2];
        rev_adj_list[b].push((a, c));
        edges.push((a, b, c));
    }

    dfs(n, &rev_adj_list, &mut visited);
    d[1] = 0;

    for i in 0..n {
        for &(a, b, c) in &edges {
            if d[a] != i64::MIN && d[a] + c > d[b] {
                d[b] = d[a] + c;
                if i == n - 1 && (visited[a] || visited[b]) {
                    println!("-1");
                    return;
                }
            }
        }
    }

    println!("{}", d[n]);
}
