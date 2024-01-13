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
        st[node] = st[left(node)] + st[right(node)];
    }
}

fn rsq(
    arr: &Vec<i64>,
    st: &mut Vec<i64>,
    node: usize,
    i: usize,
    j: usize,
    l: usize,
    r: usize,
) -> i64 {
    // println!("{:?}", ("rmq", node, l, r, i, j));
    if l > j || r < i {
        return 0;
    }
    if i <= l && r <= j {
        return st[node];
    }
    let mid = (l + r) / 2;
    return rsq(arr, st, left(node), i, j, l, mid) + rsq(arr, st, right(node), i, j, mid + 1, r);
}

fn update(arr: &Vec<i64>, st: &mut Vec<i64>, node: usize, k: usize, u: i64, l: usize, r: usize) {
    // println!("{:?}", ("rmq", node, l, r, i, j));
    if l == r {
        st[node] = u;
    } else {
        let mid = (l + r) / 2;
        if mid < k {
            update(arr, st, right(node), k, u, mid + 1, r);
        } else {
            update(arr, st, left(node), k, u, l, mid);
        }
        st[node] = st[left(node)] + st[right(node)];
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
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        if action == 1 {
            update(&arr, &mut st, 1, a - 1, b as i64, 0, n - 1);
        } else {
            println!("{}", rsq(&arr, &mut st, 1, a - 1, b - 1, 0, n - 1));
        }
    }
}

/*
8 4
3 2 4 5 1 1 5 3
2 1 4
2 5 6
1 3 1
2 1 4
*/
