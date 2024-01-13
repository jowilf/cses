use std::{
    io::stdin,
    process::{self},
};

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("");
    let nx: Vec<i32> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    let (n, x) = (nx[0], nx[1]);
    line = "".to_string();
    stdin().read_line(&mut line).expect("");
    let mut iter = line.split_whitespace();
    let mut arr: Vec<(i32, i32)> = Vec::new();
    for i in 0..n {
        arr.push((iter.next().unwrap().parse().unwrap(), i));
    }
    arr.sort();
    for i in 0..n - 2 {
        let (mut l, mut r) = (i + 1, n - 1);
        let sum = x - arr[i as usize].0;
        // println!("i={} ; sum={}", i, sum);
        while l < r {
            let lsum = arr[l as usize].0 + arr[r as usize].0;
            // println!("l={} ; r={} ; lsum={}", l, r, lsum);
            if lsum == sum && l != i && r != i {
                println!(
                    "{} {} {}",
                    arr[i as usize].1 + 1,
                    arr[l as usize].1 + 1,
                    arr[r as usize].1 + 1
                );
                process::exit(0x0100);
            } else if lsum > sum {
                r -= 1;
            } else {
                l += 1;
            }
        }
    }
    println!("IMPOSSIBLE");
}
