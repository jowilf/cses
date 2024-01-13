use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nm: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = nm[0];
    let m = nm[1];
    let k = nm[2];

    let mut adj_list = vec![Vec::new(); n + 1];

    for _ in 0..m {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let abc: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let a = abc[0];
        let b = abc[1];
        let c = abc[2];
        adj_list[a].push((b, c as u64));
    }

    let mut queue = BinaryHeap::new();
    let mut node_queue = vec![BinaryHeap::new(); n + 1];

    queue.push((Reverse(0 as u64), 1 as usize));

    while !queue.is_empty() {
        let (dist, node) = queue.pop().unwrap();
        let dnode = dist.0;
        if dnode > *node_queue[node].peek().unwrap_or(&u64::MAX) {
            continue;
        }
        for &(adj, wt) in &adj_list[node] {
            if node_queue[adj].len() < k {
                node_queue[adj].push(dnode + wt);
                queue.push((Reverse(dnode + wt), adj))
            } else if dnode + wt < *node_queue[adj].peek().unwrap() {
                node_queue[adj].pop();
                node_queue[adj].push(dnode + wt);
                queue.push((Reverse(dnode + wt), adj))
            }
        }
    }
    let mut ans = (&node_queue[n])
        .iter()
        .map(|x| x.clone())
        .collect::<Vec<u64>>();
    ans.sort();

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
