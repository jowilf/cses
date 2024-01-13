use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut arr = String::new();
    io::stdin().read_line(&mut arr).unwrap();
    let arr: Vec<i64> = arr
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut acc = vec![arr[0]];
    for i in 1..n {
        acc.push(acc[i - 1] + arr[i]);
    }

    for _ in 0..q {
        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();
        let mut iter = query.trim().split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();

        if a == 1 {
            println!("{}", acc[b - 1]);
        } else {
            println!("{}", acc[b - 1] - acc[a - 2]);
        }
    }
}
