package dynamic_programming;

import java.util.Arrays;
import java.util.Scanner;

public class GridPaths {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        sc.nextLine();

        char[][] grid = new char[n][];
        int[][] dp = new int[n][n];

        for (int i = 0; i < n; i++) {
            grid[i] = sc.nextLine().toCharArray();
            Arrays.fill(dp[i], 0);
        }

        dp[0][0] = grid[0][0] == '*' ? 0 : 1;

        for (int y = 0; y < n; y++) {
            for (int x = 0; x < n; x++) {
                if (grid[y][x] != '*') {
                    if ((x - 1) >= 0 && grid[y][x - 1] != '*') {
                        dp[y][x] += dp[y][x - 1];
                    }
                    if ((y - 1) >= 0 && grid[y - 1][x] != '*') {
                        dp[y][x] += dp[y - 1][x];
                    }
                    dp[y][x] %= 1000000007;
                }
            }
        }
        System.out.println(dp[n - 1][n - 1]);

    }
}
