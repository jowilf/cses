use std::io;

fn compute_max_depth(
    node: usize,
    parent: Option<usize>,
    adj_list: &Vec<Vec<usize>>,
    max_depth: &mut Vec<(i32, i32)>,
    ans: &mut i32,
) -> (i32, i32) {
    if  max_depth[node] != (-1, -1) {
        return max_depth[node];
    }

    let mut m1 = 0;
    let mut m2 = 0;

    for &adj in &adj_list[node] {
        if Some(adj) != parent {
            let (v1, v2) = compute_max_depth(adj, Some(node), adj_list, max_depth, ans);
            let v = v1 + 1;
            if v >= m1 {
                m2 = m1;
                m1 = v;
            } else if v > m2 {
                m2 = v;
            }
        }
    }

    *ans = (*ans).max(m1 + m2);
    max_depth[node] = (m1, m2);
    (m1, m2)
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

    let mut max_depth = vec![(-1, -1); n];
    let mut ans = 0;

    compute_max_depth(0, None, &adj_list, &mut max_depth, &mut ans);
    println!("{}", ans);
}
