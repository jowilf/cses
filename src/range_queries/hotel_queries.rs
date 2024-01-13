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
        st[node] = if arr[st[left(node)]] >= arr[st[right(node)]] {
            st[left(node)]
        } else {
            st[right(node)]
        }
    }
}

fn rmq(arr: &Vec<i32>, st: &mut Vec<usize>, v: i32, node: usize, l: usize, r: usize) -> usize {
    if l == r {
        return l;
    }
    let mid = (l + r) / 2;
    if arr[st[left(node)]] >= v {
        return rmq(arr, st, v, left(node), l, mid);
    } else {
        return rmq(arr, st, v, right(node), mid + 1, r);
    }
}
fn update(arr: &Vec<i32>, st: &mut Vec<usize>, k: usize, node: usize, l: usize, r: usize) {
    if l == r {
        st[node] = l;
    } else {
        let mid = (l + r) / 2;
        if mid >= k {
            update(arr, st, k, left(node), l, mid);
        } else {
            update(arr, st, k, right(node), mid + 1, r);
        }
        st[node] = if arr[st[left(node)]] >= arr[st[right(node)]] {
            st[left(node)]
        } else {
            st[right(node)]
        }
    }
}
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let _: usize = iter.next().unwrap().parse().unwrap();
    input = String::new();
    std::io::stdin().read_line(&mut input).expect("");
    let mut arr: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut st = vec![0; 4 * n + 1];
    build(&arr, &mut st, 1, 0, n - 1);
    input = String::new();
    std::io::stdin().read_line(&mut input).expect("");
    let rooms: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut ans = vec![];
    for r in rooms {
        if arr[st[1]] < r {
            ans.push(0);
        } else {
            let i = rmq(&arr, &mut st, r, 1, 0, n - 1);
            ans.push(i + 1);
            arr[i] -= r;
            update(&arr, &mut st, i, 1, 0, n - 1);
            // println!("arr: {:?}", arr);
        }
    }
    // println!("{:?}", ans);
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}
