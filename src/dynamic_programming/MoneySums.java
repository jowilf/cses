package dynamic_programming;

import java.util.Arrays;
import java.util.Scanner;

public class MoneySums {
    public static int N = 100 * 1000 + 2;

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        boolean[] dp = new boolean[N + 1];
        int[] coins = new int[n];
        for (int i = 0; i < n; i++) {
            coins[i] = sc.nextInt();
        }
        dp[0] = true;
        for (int c : coins) {
            for (int i = N - c; i >= 0; i--) {
                dp[i + c] |= dp[i];
            }
            // System.out.println(Arrays.toString(Arrays.copyOfRange(dp, 0, 15)));
        }
        StringBuilder stb = new StringBuilder();
        int cnt = 0;
        for (int i = 1; i < N; i++) {
            if (dp[i]) {
                stb.append(i + " ");
                cnt++;
            }
            // System.out.print(i + " ");
        }
        System.out.println(cnt);
        System.out.println(stb.toString().trim());
        // System.out.println(Arrays.toString(Arrays.copyOfRange(dp, 0, 15)));
    }
}
