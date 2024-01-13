import java.util.Arrays;
import java.util.Scanner;

public class FerrisWheel {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt(), x = sc.nextInt();
        int[] weights = new int[n];
        for (int i = 0; i < n; i++)
            weights[i] = sc.nextInt();
        Arrays.sort(weights);
        int i = 0, j = n - 1;
        int ans = 0;
        while (i <= j) {
            if (i != j && (weights[i] + weights[j] <= x)) {
                i++;
                j--;
                ans++;
            } else {
                j--;
                ans++;
            }
        }
        System.out.println(ans);
    }
}