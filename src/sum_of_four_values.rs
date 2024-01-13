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
    if n >= 4 {
        line = "".to_string();
        stdin().read_line(&mut line).expect("");
        let mut iter = line.split_whitespace();
        let mut arr: Vec<i32> = Vec::new();
        for i in 0..n {
            arr.push(iter.next().unwrap().parse().unwrap());
        }
        let mut aux: Vec<(i32, (i32, i32))> = Vec::new();
        for i in 0..n - 1 {
            for j in i + 1..n {
                aux.push((arr[i as usize] + arr[j as usize], (i, j)))
            }
        }
        aux.sort_unstable_by_key(|f| f.0);
        let nn = aux.len();
        let (mut l, mut r) = (0, nn - 1);
        while (l < r) {
            let sum = aux[l as usize].0 + aux[r as usize].0;
            if sum == x
                && aux[l as usize].1 .0 != aux[r as usize].1 .0
                && aux[l as usize].1 .0 != aux[r as usize].1 .1
                && aux[l as usize].1 .1 != aux[r as usize].1 .0
                && aux[l as usize].1 .1 != aux[r as usize].1 .1
            {
                println!(
                    "{} {} {} {}",
                    aux[l as usize].1 .0 + 1,
                    aux[l as usize].1 .1 + 1,
                    aux[r as usize].1 .0 + 1,
                    aux[r as usize].1 .1 + 1,
                );
                process::exit(0x0100);
            } else if sum < x {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
    println!("IMPOSSIBLE");
}
