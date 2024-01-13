package dynamic_programming;

import java.util.Arrays;
import java.util.Scanner;

public class BookShop {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt(), X = sc.nextInt();
        int[] h = new int[n];
        int[] s = new int[n];
        for (int i = 0; i < n; i++)
            h[i] = sc.nextInt();
        for (int i = 0; i < n; i++)
            s[i] = sc.nextInt();
        int[] dp = new int[X + 1];
        Arrays.fill(dp, Integer.MIN_VALUE);
        dp[0] = 0;
        int ans = 0;
        for (int k = 0; k < n; k++) {
            for (int x = X - h[k]; x >= 0; x--) {
                if (dp[x] != Integer.MIN_VALUE) {
                    dp[x + h[k]] = Math.max(dp[x + h[k]], dp[x] + s[k]);
                    ans = Math.max(ans, dp[x + h[k]]);
                }
            }
        }
        // System.out.println(Arrays.toString(dp));
        System.out.println(ans);
    }
}
