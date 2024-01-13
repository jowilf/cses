import java.util.ArrayList;
import java.util.Scanner;

public class TowerofHanoi {

    static class Stack {
        int id;
        ArrayList<Integer> arr;

        public Stack(int id, ArrayList<Integer> arr) {
            this.id = id;
            this.arr = arr;
        }
    }

    static ArrayList<Integer[]> ans = new ArrayList<>();

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        move(new Stack(1, new ArrayList<Integer>() {
            {
                for (int i = 1; i <= n; i++)
                    add(i);
            }
        }), new Stack(3, new ArrayList<Integer>()), new Stack(2, new ArrayList<Integer>()), n);
        StringBuilder stb = new StringBuilder();
        stb.append(ans.size() + "\n");
        for (Integer[] v : ans)
            stb.append(v[0] + " " + v[1] + "\n");
        System.out.print(stb);
    }

    static void move(Stack source, Stack target, Stack auxiliary, int n) {
        if (n > 0) {
            move(source, auxiliary, target, n - 1);
            target.arr.add(source.arr.remove(source.arr.size() - 1));
            ans.add(new Integer[] { source.id, target.id });
            move(auxiliary, target, source, n - 1);
        }
    }
}
