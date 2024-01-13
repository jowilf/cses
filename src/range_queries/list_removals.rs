fn left(i: usize) -> usize {
    2 * i
}
fn right(i: usize) -> usize {
    2 * i + 1
}
fn build(arr: &Vec<i64>, st: &mut Vec<(i64, i64, i64)>, node: usize, l: usize, r: usize) {
    // println!("{:?}", ("build", node, l, r));
    if l == r {
        st[node] = (arr[l], l as i64, 0);
    } else {
        let mid = (l + r) / 2;
        build(arr, st, left(node), l, mid);
        build(arr, st, right(node), mid + 1, r);
        let max_idx = if st[left(node)].1 >= st[right(node)].1 {
            st[left(node)].1
        } else {
            st[right(node)].1
        };
        st[node] = (0, max_idx, 0);
    }
}

fn query(
    arr: &Vec<i64>,
    st: &mut Vec<(i64, i64, i64)>,
    node: usize,
    k: usize,
    l: usize,
    r: usize,
) -> i64 {
    println!("{:?}", ("query", st[node],k, l, r));
    if l == r {
        return st[node].0;
    }
    if st[node].2 != 0 {
        st[left(node)].1 += st[node].2;
        st[left(node)].2 += st[node].2;

        st[right(node)].1 += st[node].2;
        st[right(node)].2 += st[node].2;

        st[node].2 = 0;
    }
    let mid = (l + r) / 2;
    if st[left(node)].1 >= (k as i64) {
        return query(arr, st, left(node), k, l, mid);
    }
    return query(arr, st, right(node), k, mid + 1, r);
}

fn update(
    arr: &Vec<i64>,
    st: &mut Vec<(i64, i64, i64)>,
    node: usize,
    i: usize,
    j: usize,
    u: i64,
    l: usize,
    r: usize,
) {
    println!("{:?}", ("update", l, r,u, i, j));
    if l > j || r < i {
        return;
    } else if l >= i && r <= j {
        st[node].1 += u;
        st[node].2 += u;
    } else {
        let mid = (l + r) / 2;
        update(arr, st, left(node), i, j, u, l, mid);
        update(arr, st, right(node), i, j, u, mid + 1, r);
        let max_idx = if st[left(node)].1 >= st[right(node)].1 {
            st[left(node)].1
        } else {
            st[right(node)].1
        };
        st[node].1 = max_idx;
    }
}
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    input = String::new();
    std::io::stdin().read_line(&mut input).expect("");
    let arr: Vec<i64> = input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    input = String::new();
    std::io::stdin().read_line(&mut input).expect("");
    let queries: Vec<usize> = input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut st = vec![(0, 0, 0); 4 * n + 1];
    build(&arr, &mut st, 1, 0, n - 1);
    // println!("{:?}", st);
    let mut ans = vec![];
    for &q in &queries {
        println!("{:?}", st);
        ans.push(query(&arr, &mut st, 1, q - 1, 0, n - 1)); // query the value in the index
        // break;
        update(&arr, &mut st, 1, q - 1, q - 1, -((n as i64) + 1), 0, n - 1);
        update(&arr, &mut st, 1, q, n - 1, -1, 0, n - 1);
    }
    println!("{:?}", ans);
}

/*
8 3
3 2 4 5 1 1 5 3
2 4
1 2 5 1
2 4
*/
