import java.util.Scanner;

public class IncreasingArray {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt(), l = sc.nextInt();
        long ans = 0;
        for (int i = 1; i < n; i++) {
            int v = sc.nextInt();
            if (v < l)
                ans += l - v;
            l = Math.max(l, v);
        }
        System.out.println(ans);
    }
}
