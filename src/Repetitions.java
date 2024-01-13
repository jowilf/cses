import java.util.Scanner;

public class Repetitions {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        char c = '-';
        int ans = 0, cnt = 0;
        String text = sc.next();
        for (int i = 0; i < text.length(); i++) {

            if (text.charAt(i) == c)
                cnt++;
            else {
                cnt = 1;
                c = text.charAt(i);
            }
            ans = Math.max(ans, cnt);
        }
        System.out.println(ans);
    }
}
