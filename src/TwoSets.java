import java.util.Scanner;

public class TwoSets {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        long n = sc.nextLong();
        long s = (n * (n + 1)) / 2;
        if (s % 2 == 0) {
            long ss2 = s / 2;
            StringBuilder stb = new StringBuilder();
            System.out.println("YES");
            long r = n;
            long ls = 0;
            while ((ss2 - ls) > r) {
                ls += r;
                r--;
            }
            stb.append((n - r + 1) + "\n");
            for (long i = r + 1; i <= n; i++)
                stb.append(i + " ");
            stb.append((ss2 - ls) + "\n");
            stb.append((r - 1) + "\n");
            for (int i = 1; i <= r; i++)
                if (i != (ss2 - ls))
                    stb.append(i + " ");
            System.out.println(stb);
        } else {
            System.out.println("NO");
        }
    }
}
