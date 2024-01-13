const MAX_K_POWER: usize = 30;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nq: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = nq[0];
    let q = nq[1];
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let succ: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut dp = vec![[0; MAX_K_POWER]; n + 1];
    for k in 0..MAX_K_POWER {
        for i in 1..=n {
            if k == 0 {
                dp[i][k] = succ[i - 1];
            } else {
                dp[i][k] = dp[dp[i][k - 1]][k - 1];
            }
        }
    }

    for _ in 0..q {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let mut x: usize = iter.next().unwrap().parse().unwrap();
        let mut k: i64 = iter.next().unwrap().parse().unwrap();
        let mut pow = 0;
        while k > 0 {
            if k % 2 == 1 {
                x = dp[x][pow];
            }
            k /= 2;
            pow += 1;
        }
        println!("{}", x);
    }
}
