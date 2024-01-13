import java.util.ArrayList;
import java.util.Scanner;

public class Permutations {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        if (n == 2 || n == 3)
            System.out.println("NO SOLUTION");
        else {
            StringBuilder stb = new StringBuilder();
            for (int i = 2; i <= n; i += 2)
                stb.append(i + " ");
            for (int i = 1; i <= n; i += 2)
                stb.append(i + ((i + 2) > n ? "" : " "));
            System.out.println(stb);
        }
    }
}
