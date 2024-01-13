import java.util.ArrayList;
import java.util.Scanner;

public class AppleDivision {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int[] values = new int[n];
        for (int i = 0; i < n; i++)
            values[i] = sc.nextInt();
        System.out.println(solve(values, 0, 0, 0));
    }

    static long solve(int[] values, int pos, long s1, long s2) {
        // System.out.println(s1 + " === "+ s2);
        if (pos == values.length)
            return Math.abs(s2 - s1);
        return Math.min(solve(values, pos + 1, s1 + values[pos], s2), solve(values, pos + 1, s1, s2 + values[pos]));
    }
}
