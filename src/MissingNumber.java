import java.math.BigInteger;
import java.util.Scanner;

public class MissingNumber {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        long n = sc.nextLong();
        long sum = n * (n + 1) / 2;
        for (int i = 0; i < n - 1; i++)
            sum -= sc.nextLong();
        System.out.println(sum);
    }
}
