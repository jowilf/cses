use std::{cmp::min, i64::MAX, io::stdin};
fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("");
    let mut iter = line.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let m: i32 = iter.next().unwrap().parse().unwrap();
    let q: i32 = iter.next().unwrap().parse().unwrap();
    let mut d = [[MAX; 501]; 501];
    for i in 0..n as usize {
        d[i][i] = 0;
    }
    for _ in 0..m {
        line = "".to_string();
        stdin().read_line(&mut line).expect("");
        let mut iter = line.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        let c: i64 = iter.next().unwrap().parse().unwrap();
        let ai = (a - 1) as usize;
        let bi = (b - 1) as usize;
        d[ai][bi] = min(d[ai][bi], c);
        d[bi][ai] = min(d[bi][ai], c);
    }

    for k in 0..n as usize {
        for i in 0..n as usize {
            for j in 0..n as usize {
                d[i][j] = min(d[i][j], d[i][k] + d[k][j]);
            }
        }
    }

    // println!("{:?}", d);

    for _ in 0..q {
        line = "".to_string();
        stdin().read_line(&mut line).expect("");
        let mut iter = line.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        let ai = (a - 1) as usize;
        let bi = (b - 1) as usize;
        println!("{}", if d[ai][bi] != MAX { d[ai][bi] } else { -1 });
    }
}
