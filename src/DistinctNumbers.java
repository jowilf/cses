import java.util.Arrays;
import java.util.HashSet;
import java.util.Scanner;
import java.util.Set;

public class DistinctNumbers {
    public static void main(String[] args) {
        impl2();
    }

    static void impl1() {
        // With Set
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        Set<Integer> values = new HashSet<>();
        for (int i = 0; i < n; i++) {
            values.add(sc.nextInt());
        }
        System.out.println(values.size());
    }

    static void impl2() {
        // Without Set
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int[] values = new int[n];
        for (int i = 0; i < n; i++) {
            values[i] = sc.nextInt();
        }
        Arrays.sort(values);
        int cnt = 0;
        int i = 0;
        while (i < n) {
            cnt++;
            while (i < (n - 1) && values[i] == values[i + 1])
                i++;
            i++;
        }
        System.out.println(cnt);
    }
}