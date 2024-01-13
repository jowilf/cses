use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("-error-");
    let n: i32 = line.lines().next().unwrap().parse().expect("Invalid input");
    line = "".to_string();
    stdin().read_line(&mut line).expect("-error-");
    let arr: Vec<i64> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    let mut dp: [[(i64, i64); 5000]; 5000] = [[(0, 0); 5000]; 5000];
    for i in 0..(n as usize) {
        dp[i][i] = (arr[i], 0);
    }
    for l in (0..(n as usize)).rev() {
        for r in l + 1..(n as usize) {
            let s1 = dp[l + 1][r];
            let s2 = dp[l][r - 1];
            if (arr[l] + s1.1) > arr[r] + s2.1 {
                dp[l][r] = (arr[l] + s1.1, s1.0);
            } else {
                dp[l][r] = (arr[r] + s2.1, s2.0);
            }
        }
    }
    println!("{}", dp[0][(n - 1) as usize].0)
}
