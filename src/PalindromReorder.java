import java.util.Scanner;

public class PalindromReorder {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        char[] text = sc.next().toCharArray();
        int[] counter = new int[26];
        for (int i = 0; i < text.length; i++) {
            counter[text[i] - 'A']++;
        }
        int middle = -1;
        int l = 0, r = text.length - 1;
        for (int i = 0; i < counter.length; i++) {
            if (counter[i] % 2 != 0) {
                if (middle != -1) {
                    System.out.println("NO SOLUTION");
                    System.exit(0);
                }
                middle = i;
                text[text.length / 2] = (char) ('A' + middle);
            }
            for (int j = 0; j < counter[i] / 2; j++) {
                text[r] = (char) (i + 'A');
                text[l] = (char) (i + 'A');
                l += 1;
                r -= 1;
            }
        }
        System.out.println(text);
    }
}
