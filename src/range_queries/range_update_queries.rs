fn left(i: usize) -> usize {
    2 * i
}
fn right(i: usize) -> usize {
    2 * i + 1
}
fn build(arr: &Vec<i64>, st: &mut Vec<i64>, node: usize, l: usize, r: usize) {
    // println!("{:?}", ("build", node, l, r));
    if l == r {
        st[node] = arr[l];
    } else {
        let mid = (l + r) / 2;
        build(arr, st, left(node), l, mid);
        build(arr, st, right(node), mid + 1, r);
        st[node] = 0;
    }
}

fn query(arr: &Vec<i64>, st: &mut Vec<i64>, node: usize, k: usize, l: usize, r: usize) -> i64 {
    if l == r {
        return st[node];
    }
    if st[node] > 0 {
        st[left(node)] += st[node];
        st[right(node)] += st[node];
        st[node] = 0;
    }
    let mid = (l + r) / 2;
    if mid >= k {
        return query(arr, st, left(node), k, l, mid);
    }
    return query(arr, st, right(node), k, mid + 1, r);
}

fn update(
    arr: &Vec<i64>,
    st: &mut Vec<i64>,
    node: usize,
    i: usize,
    j: usize,
    u: i64,
    l: usize,
    r: usize,
) {
    // println!("{:?}", ("update", node, l, r, i, j));
    if l > j || r < i {
        return;
    } else if l >= i && r <= j {
        st[node] += u;
    } else {
        let mid = (l + r) / 2;
        update(arr, st, left(node), i, j, u, l, mid);
        update(arr, st, right(node), i, j, u, mid + 1, r);
    }
}
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    input = String::new();
    std::io::stdin().read_line(&mut input).expect("");
    let arr: Vec<i64> = input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut st = vec![0; 4 * n + 1];
    build(&arr, &mut st, 1, 0, n - 1);
    // println!("{:?}", st);
    for _ in 0..q {
        input = String::new();
        std::io::stdin().read_line(&mut input).expect("");
        let mut iter = input.split_whitespace();
        let action: i64 = iter.next().unwrap().parse().unwrap();

        if action == 1 {
            let a: usize = iter.next().unwrap().parse().unwrap();
            let b: usize = iter.next().unwrap().parse().unwrap();
            let u: i64 = iter.next().unwrap().parse().unwrap();
            update(&arr, &mut st, 1, a - 1, b - 1, u, 0, n - 1);
        } else {
            let k: usize = iter.next().unwrap().parse().unwrap();
            println!("{}", query(&arr, &mut st, 1, k - 1, 0, n - 1));
        }
    }
}

/*
8 3
3 2 4 5 1 1 5 3
2 4
1 2 5 1
2 4
*/
