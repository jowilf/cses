use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("-error-");
    let n: i32 = line.lines().next().unwrap().parse().expect("Invalid input");
    let sum = ((1 + n) * n) / 2;
    let mut dp = [0 as i64; (1 + 500) * 125 + 1];
    let modx = 1000000007;
    dp[0] = 1;
    if sum % 2 == 0 {
        let mid = sum / 2;
        for c in 1..=n {
            for i in (0..=mid - c).rev() {
                // println!("{} {} {}", c, i, mid);
                dp[(i + c) as usize] += dp[i as usize];
                if dp[(i + c) as usize] >= modx {
                    dp[(i + c) as usize] %= modx;
                }
            }
        }
        // println!("{} {:?}",1 , dp[mid as usize] %2);
        println!("{}", (dp[mid as usize] * ((modx + 1) / 2)) % modx);
    } else {
        println!("0");
    }
}
