import java.lang.Thread.State;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Scanner;

public class ChessBoardQueens {

    static class Point {
        int x;
        int y;

        public Point(int x, int y) {
            this.x = x;
            this.y = y;
        }
    }

    static int count = 0;
    static char[][] grid = new char[8][8];

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        for (int i = 0; i < 8; i++)
            grid[i] = sc.next().toCharArray();
        search(new ArrayList<>(), 0);
        System.out.println(count);
        // for (int i = 0; i < 8; i++)
        // System.out.println(Arrays.toString(grid[i]));
    }

    static void search(ArrayList<Point> stack, int x) {
        if (stack.size() == 8)
            count += 1;
        else if (stack.size() < 8) {
            for (int y = 0; y < 8; y++) {
                boolean isValid = true;
                for (Point p : stack) {
                    if (p.x == x || p.y == y || Math.abs(p.x - x) == Math.abs(p.y - y)) {
                        isValid = false;
                        break;
                    }
                }
                if (isValid && grid[y][x] != '*') {
                    stack.add(new Point(x, y));
                    search(stack, x + 1);
                    stack.remove(stack.size() - 1);
                }
            }
        }
    }
}