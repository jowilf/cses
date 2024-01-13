import java.math.BigInteger;
import java.util.Scanner;

public class TwoKnights {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        long k = sc.nextLong();
        for (int i = 1; i <= k; i++) {
            BigInteger v = new BigInteger(i + "");
            System.out.println(v.pow(2).multiply(v.pow(2).subtract(new BigInteger("1"))).divide(new BigInteger("2"))
                    .subtract(v.subtract(
                            new BigInteger("2"))
                            .multiply(v.subtract(new BigInteger("1"))).multiply(new BigInteger("4"))));
        }
    }
}