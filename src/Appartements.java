import java.util.Arrays;
import java.util.Scanner;

public class Appartements {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt(), m = sc.nextInt(), k = sc.nextInt();
        int[] applicants = new int[n], apartements = new int[m];
        for (int i = 0; i < n; i++)
            applicants[i] = sc.nextInt();
        for (int i = 0; i < m; i++)
            apartements[i] = sc.nextInt();
        Arrays.sort(applicants);
        Arrays.sort(apartements);
        int i = 0, j = 0, ans = 0;
        while (i < n && j < m) {
            if (Math.abs(applicants[i] - apartements[j]) <= k) {
                ans++;
                j++;
                i++;
            } else if ((applicants[i] - k) > apartements[j])
                j++;
            else
                i++;
        }
        System.out.println(ans);
    }
}
