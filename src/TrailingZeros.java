import java.util.Scanner;

public class TrailingZeros {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt(), k = 1, ans = 0;
        while (Math.pow(5, k) <= n) {
            ans += (int) (n / Math.pow(5, k));
            k += 1;
        }
        System.out.println(ans);
    }
}
