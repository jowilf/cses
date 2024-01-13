package dynamic_programming;

import java.util.Scanner;

public class EditDistance {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        String first = sc.nextLine(), second = sc.nextLine();
        int n = first.length(), m = second.length();
        int[][] dp = new int[n + 1][m + 1];
        dp[0][0] = 0;
        for (int i = 0; i <= n; i++) {
            for (int j = 0; j <= m; j++) {
                if (i != 0 || j != 0)
                    dp[i][j] = Integer.MAX_VALUE;
                if ((i - 1) >= 0)
                    dp[i][j] = Math.min(dp[i - 1][j] + 1, dp[i][j]);
                if ((j - 1) >= 0)
                    dp[i][j] = Math.min(dp[i][j - 1] + 1, dp[i][j]);
                if ((i - 1) >= 0 && (j - 1) >= 0)
                    dp[i][j] = Math.min(dp[i - 1][j - 1] + (first.charAt(i - 1) == second.charAt(j - 1) ? 0 : 1),
                            dp[i][j]);
            }
        }
        System.out.println(dp[n][m]);
        // for (int i = 0; i < dp.length; i++) {
        // System.out.println(Arrays.toString(dp[i]));
        // }
    }
}
