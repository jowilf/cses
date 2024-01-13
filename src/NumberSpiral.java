import java.util.Scanner;

public class NumberSpiral {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int t = sc.nextInt();
        StringBuilder stb = new StringBuilder();
        while (t-- > 0) {
            long y = sc.nextLong(), x = sc.nextLong(), ans = 0;
            if (x >= y) {
                ans = x % 2 == 0 ? x * x - (x - 1) * 2 + y - 1 : (x * x) - y + 1;
            } else {
                ans = y % 2 != 0 ? y * y - (y - 1) * 2 + x - 1 : (y * y) - x + 1;
            }
            stb.append(ans + (t > 0 ? "\n" : ""));
        }
        System.out.println(stb);
    }
}
