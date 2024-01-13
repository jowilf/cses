fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut edges = Vec::new();
    let mut cluster = (0..=n).collect::<Vec<usize>>();
    let mut size = vec![1; n + 1];

    for _ in 0..m {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        let wt: i64 = iter.next().unwrap().parse().unwrap();
        edges.push((u, v, wt));
    }

    edges.sort_by_key(|&(_, _, wt)| wt);

    let mut ans = 0;
    for &(u, v, wt) in &edges {
        if !same(u, v, &mut cluster) {
            ans += wt;
            unite(u, v, &mut cluster, &mut size);
        }
    }

    if size[find(1, &mut cluster)] == n {
        println!("{}", ans);
    } else {
        println!("IMPOSSIBLE");
    }
}

fn same(u: usize, v: usize, cluster: &mut [usize]) -> bool {
    find(u, cluster) == find(v, cluster)
}

fn find(x: usize, cluster: &mut [usize]) -> usize {
    if cluster[x] == x {
        return x;
    }
    cluster[x] = find(cluster[x], cluster);
    cluster[x]
}

fn unite(u: usize, v: usize, cluster: &mut [usize], size: &mut [usize]) {
    let root_u = find(u, cluster);
    let root_v = find(v, cluster);
    if size[root_u] > size[root_v] {
        cluster[root_v] = root_u;
        size[root_u] += size[root_v];
        size[root_v] = 0;
    } else {
        cluster[root_u] = root_v;
        size[root_v] += size[root_u];
        size[root_u] = 0;
    }
}
