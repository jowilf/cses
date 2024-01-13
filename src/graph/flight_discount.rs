use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    dist: u64,
    max_weight: u64,
    id: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| other.max_weight.cmp(&self.max_weight))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(start: usize, end: usize, adj_list: &[Vec<(usize, u64)>]) -> u64 {
    let n = adj_list.len();
    let mut distances = vec![std::u64::MAX; n];
    distances[start] = 0;

    let mut visited = vec![false; n];
    let mut queue = BinaryHeap::new();

    queue.push(Node {
        dist: 0,
        max_weight: 0,
        id: start,
    });

    while let Some(Node {
        dist,
        max_weight,
        id,
    }) = queue.pop()
    {
        if visited[id] {
            continue;
        }

        visited[id] = true;

        if id == end {
            return distances[end];
        }

        for &(adj, weight) in &adj_list[id] {
            let new_dist = if weight > max_weight {
                dist + (max_weight - max_weight / 2) + weight / 2
            } else {
                dist + weight
            };

            if new_dist <= distances[adj] {
                distances[adj] = new_dist;
                queue.push(Node {
                    dist: new_dist,
                    max_weight: max_weight.max(weight),
                    id: adj,
                });
            }
        }
    }
    std::u64::MAX
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
    let mut rev_adj_list = vec![Vec::new(); n + 1];

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
        rev_adj_list[b].push((a, c as u64));
    }

    let result = dijkstra(1, n, &adj_list).min(dijkstra(n, 1, &rev_adj_list));
    println!("{}", result);
}
