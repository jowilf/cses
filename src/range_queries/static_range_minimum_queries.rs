fn left(i: usize) -> usize {
    2 * i
}
fn right(i: usize) -> usize {
    2 * i + 1
}
fn build(arr: &Vec<i32>, st: &mut Vec<usize>, node: usize, l: usize, r: usize) {
    // println!("{:?}", ("build", node, l, r));
    if l == r {
        st[node] = l;
    } else {
        let mid = (l + r) / 2;
        build(arr, st, left(node), l, mid);
        build(arr, st, right(node), mid + 1, r);
        st[node] = if arr[st[left(node)]] < arr[st[right(node)]] {
            st[left(node)]
        } else {
            st[right(node)]
        }
    }
}

fn rmq(
    arr: &Vec<i32>,
    st: &mut Vec<usize>,
    node: usize,
    i: usize,
    j: usize,
    l: usize,
    r: usize,
) -> i32 {
    // println!("{:?}", ("rmq", node, l, r, i, j));
    if l > j || r < i {
        return -1;
    }
    if i <= l && r <= j {
        return st[node] as i32;
    }
    let mid = (l + r) / 2;
    let idx1 = rmq(arr, st, left(node), i, j, l, mid);
    let idx2 = rmq(arr, st, right(node), i, j, mid + 1, r);
    if idx1 == -1 {
        return idx2;
    }
    if idx2 == -1 {
        return idx1;
    }
    return if arr[idx1 as usize] < arr[idx2 as usize] { idx1 } else { idx2 };
}
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    input = String::new();
    std::io::stdin().read_line(&mut input);
    let arr: Vec<i32> = input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut st = vec![0; 4 * n + 1];
    build(&arr, &mut st, 1, 0, n - 1);
    // println!("{:?}", st);
    for i in 0..q {
        input = String::new();
        std::io::stdin().read_line(&mut input);
        let mut iter = input.split_whitespace();
        let i: usize = iter.next().unwrap().parse().unwrap();
        let j: usize = iter.next().unwrap().parse().unwrap();
        println!("{}", arr[rmq(&arr, &mut st, 1, i - 1, j - 1, 0, n - 1) as usize])
    }
}

/*
31 1
1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
1 30
1 2
1 3
1 4

 */
