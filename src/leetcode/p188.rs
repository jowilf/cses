use std::cmp;

fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    let mut dp = [[-1; 1001]; 101];
    return solve(&prices, prices.len() as i32, k, &mut dp);
}

fn solve(prices: &Vec<i32>, n: i32, k: i32, dp: &mut [[i32; 1001]; 101]) -> i32 {
    if k == 0 || n < 2 {
        return 0;
    }
    if dp[k as usize][n as usize] != -1 {
        return dp[k as usize][n as usize];
    }
    let mut l_max: i32 = 0;
    let mut ans = 0;
    for i in (0..n as usize).rev() {
        ans = cmp::max(
            ans,
            solve(prices, i as i32, k - 1, dp) + cmp::max(0, l_max - prices[i]),
        );
        l_max = cmp::max(prices[i], l_max);
    }
    dp[k as usize][n as usize] = ans;
    return ans;
}

fn main() {
    println!("{}", max_profit(2, vec![3, 2, 6, 5, 0, 3]))
}
