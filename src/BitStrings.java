import java.util.Scanner;

public class BitStrings {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        System.out.println(pow(2, n, 1000000007));
    }

    static long pow(long x, int y, int k) {
        long ans = 1;
        while (y > 0) {
            if (y % 2 == 1)
                ans = (ans * x) % k;
            y = y / 2;
            x *= x;
            x %= k;
            // System.out.println(x);
        }
        return ans;
    }
}