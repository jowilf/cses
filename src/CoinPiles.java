import java.util.Scanner;

public class CoinPiles {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        StringBuilder stb = new StringBuilder();
        int t = sc.nextInt();
        for (int i = 0; i < t; i++) {
            int a = sc.nextInt(), b = sc.nextInt();
            int max = Math.max(a, b);
            int min = Math.min(a, b);
            int d = max - min;
            if ((min + max) % 3 == 0 && (max - 2 * d) == (min - d) && (min - d) >= 0) {
                stb.append("YES" + "\n");
            } else
                stb.append("NO" + "\n");
        }
        System.out.print(stb);
    }
}
