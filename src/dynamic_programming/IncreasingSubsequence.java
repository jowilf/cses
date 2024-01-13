package dynamic_programming;

import java.util.Scanner;

public class IncreasingSubsequence {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt(), len = 0, mid, lo, hi;
        int[] arr = new int[n];
        int[] dp = new int[n];
        for (int i = 0; i < arr.length; i++) {
            arr[i] = sc.nextInt();
        }
        dp[0] = arr[0];
        for (int i = 1; i < dp.length; i++) {
            lo = 0;
            hi = len;
            while (lo <= hi) {
                mid = (lo + hi) / 2;
                if (arr[i] > dp[mid])
                    lo = mid + 1;
                else
                    hi = mid - 1;
            }
            dp[lo] = arr[i];
            len = Math.max(len, lo);
            // System.out.println(Arrays.toString(dp));
        }
        System.out.println(len + 1);
    }
}
