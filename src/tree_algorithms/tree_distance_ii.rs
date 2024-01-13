use std::io;

fn dfs1(
    node: usize,
    parent: Option<usize>,
    adj_list: &Vec<Vec<usize>>,
    dist: &mut Vec<(i64, i64)>,
    ans: &mut Vec<i64>,
) -> (i64, i64) {
    let (mut sum, mut count) = (0, 0);

    for &adj in &adj_list[node] {
        if Some(adj) != parent {
            let (s, c) = dfs1(adj, Some(node), adj_list, dist, ans);
            sum += s + c + 1;
            count += c + 1;
        }
    }

    dist[node] = (sum, count);
    (sum, count)
}

fn dfs2(
    n: i64,
    node: usize,
    parent: Option<usize>,
    adj_list: &Vec<Vec<usize>>,
    dist: &mut Vec<(i64, i64)>,
    ans: &mut Vec<i64>,
) {
    if parent.is_some() {
        let (s, c) = dist[node];
        ans[node] = s + ans[parent.unwrap()] - (s + c + 1) + (n - c - 1);
    } else {
        ans[node] = dist[node].0;
    }

    for &adj in &adj_list[node] {
        if Some(adj) != parent {
            dfs2(n, adj, Some(node), adj_list, dist, ans);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut adj_list = vec![Vec::new(); n];

    for _ in 0..n - 1 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let nums: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid input"))
            .collect();

        let i = nums[0] - 1;
        let j = nums[1] - 1;

        adj_list[i].push(j);
        adj_list[j].push(i);
    }

    let mut dist = vec![(0, 0); n];
    let mut ans = vec![0; n];

    dfs1(0, None, &adj_list, &mut dist, &mut ans);
    // println!("{:?}", max_depth);
    dfs2(n as i64, 0, None, &adj_list, &mut dist, &mut ans);
    // println!("{:?}", max_depth);
    println!(
        "{}",
        ans.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
