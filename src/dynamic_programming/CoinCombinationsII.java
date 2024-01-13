package dynamic_programming;

import java.util.Arrays;
import java.util.Scanner;

public class CoinCombinationsII {
    static int N = 1000001;

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt(), x = sc.nextInt();
        int[] dp = new int[N];
        int[] coins = new int[n];
        Arrays.fill(dp, 0);
        dp[0] = 1;
        for (int i = 0; i < n; i++) {
            coins[i] = sc.nextInt();
        }
        for (int coin : coins) {
            for (int i = 1; i <= x; i++) {
                if ((i - coin) >= 0) {
                    dp[i] += dp[i - coin];
                    dp[i] %= 1000000007;
                }
            }
        }
        // System.out.println(Arrays.toString(Arrays.copyOfRange(dp, 1, x + 1)));
        System.out.println(dp[x]);
    }
}
