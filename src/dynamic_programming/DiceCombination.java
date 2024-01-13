package dynamic_programming;

import java.util.Scanner;

public class DiceCombination {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int[] cnt = new int[n + 1];
        for (int i = 0; i <= n; i++)
            cnt[i] = (i >= 1 && i <= 6) ? 1 : 0;
        for (int i = 1; i <= n; i++) {
            for (int j = 1; j <= 6; j++) {
                if ((i - j) > 0) {
                    cnt[i] += cnt[i - j];
                    cnt[i] %= 1000000007;
                }
            }
        }
        System.out.println(cnt[n]);
    }
}
