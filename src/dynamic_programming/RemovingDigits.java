package dynamic_programming;

import java.util.Arrays;
import java.util.Scanner;

public class RemovingDigits {
    static Integer[] dp = new Integer[1000001];

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int dp[] = new int[1000001];
        Arrays.fill(dp, Integer.MAX_VALUE);
        dp[0] = 0;
        for (int x = 1; x <= n; x++) {
            String xstr = String.valueOf(x);
            for (int i = 0; i < xstr.length(); i++) {
                char c = xstr.charAt(i);
                if (c != '0') {
                    dp[x] = Math.min(dp[x], 1 + dp[x - (c - '0')]);
                }
            }
        }
        // System.out.println(Arrays.toString(Arrays.copyOfRange(dp, 1, n)));
        System.out.println(dp[n]);
        // System.out.println(solve(n));
    }

    public static int solve(int x) {
        // System.out.println("solve " + x);
        if (x == 0)
            return 0;
        if (dp[x] != null)
            return dp[x];
        int ans = Integer.MAX_VALUE;
        String xstr = String.valueOf(x);
        for (int i = 0; i < xstr.length(); i++) {
            char c = xstr.charAt(i);
            if (c != '0') {
                int s = solve(x - (c - '0'));
                ans = Math.min(ans, s + (s == Integer.MAX_VALUE ? 0 : 1));
            }
        }
        dp[x] = ans;
        return ans;
    }
}
