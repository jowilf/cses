use std::io;

fn compute_max_depth1(
    node: usize,
    parent: Option<usize>,
    adj_list: &Vec<Vec<usize>>,
    max_depth: &mut Vec<(i32, i32)>,
    ans: &mut i32,
) -> (i32, i32) {
    let mut m1 = 0;
    let mut m2 = 0;

    for &adj in &adj_list[node] {
        if Some(adj) != parent {
            let (v1, _v2) = compute_max_depth1(adj, Some(node), adj_list, max_depth, ans);
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

fn compute_max_depth2(
    node: usize,
    parent: Option<usize>,
    adj_list: &Vec<Vec<usize>>,
    max_depth: &mut Vec<(i32, i32)>,
    ans: &mut i32,
) {
    if parent.is_some() {
        let parent_max_l;

        let (m1p,  m2p) = max_depth[parent.unwrap()];
        let (mut m1, mut m2) = max_depth[node];

        if m1p == m1 + 1 {
            parent_max_l = m2p + 1;
        } else {
            parent_max_l = m1p + 1;
        }

        if parent_max_l >= m1 {
            m2 = m1;
            m1 = parent_max_l;
        } else if parent_max_l > m2 {
            m2 = parent_max_l;
        }
        max_depth[node] = (m1, m2)
    }

    for &adj in &adj_list[node] {
        if Some(adj) != parent {
            compute_max_depth2(adj, Some(node), adj_list, max_depth, ans);
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

    let mut max_depth = vec![(-1, -1); n];
    let mut ans = 0;

    compute_max_depth1(0, None, &adj_list, &mut max_depth, &mut ans);
    // println!("{:?}", max_depth);
    compute_max_depth2(0, None, &adj_list, &mut max_depth, &mut ans);
    // println!("{:?}", max_depth);
    println!(
        "{}",
        max_depth
            .into_iter()
            .map(|x| x.0.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
