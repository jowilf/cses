package graph;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.LinkedList;
import java.util.Scanner;
import java.util.StringTokenizer;

public class CountingRooms {
    public static void main(String[] args) {
        FastReader sc = new FastReader();
        int n = sc.nextInt(), m = sc.nextInt();
        char[][] grid = new char[n][m];
        boolean[][] visited = new boolean[n][m];
        for (int i = 0; i < n; i++) {
            grid[i] = sc.nextLine().toCharArray();
            for (int j = 0; j < m; j++) {
                visited[i][j] = false;
            }
        }
        int ans = 0;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < m; j++) {
                if (grid[i][j] == '.' && !visited[i][j]) {
                    // System.out.println("i: " + i + "; j:" + j);
                    visit(i, j, n, m, grid, visited);
                    ans++;
                }
            }
        }
        System.out.println(ans);
    }

    public static void visit(int x, int y, int n, int m, char[][] grid, boolean[][] visited) {
        // System.out.println("visit i: " + x + "; j:" + y);
        int[] dx = new int[] { 1, -1, 0, 0 };
        int[] dy = new int[] { 0, 0, 1, -1 };
        int xx, yy;
        LinkedList<P> stack = new LinkedList<>();
        stack.addFirst(new P(x, y));
        while (!stack.isEmpty()) {
            P next = stack.poll();
            x = next.x;
            y = next.y;
            visited[x][y] = true;
            for (int i = 0; i < dy.length; i++) {
                xx = x + dx[i];
                yy = y + dy[i];
                if (xx >= 0 && xx < n && yy >= 0 && yy < m && grid[xx][yy] == '.' && !visited[xx][yy])
                    stack.addLast(new P(xx, yy));
            }
        }
    }

    static class P {
        int x, y;

        public P(int x, int y) {
            this.x = x;
            this.y = y;
        }
    }

    static class FastReader {
        BufferedReader br;
        StringTokenizer st;

        public FastReader() {
            br = new BufferedReader(new InputStreamReader(System.in));
        }

        String next() {
            while (st == null || !st.hasMoreTokens()) {
                try {
                    st = new StringTokenizer(br.readLine());
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
            return st.nextToken();
        }

        int nextInt() {
            return Integer.parseInt(next());
        }

        long nextLong() {
            return Long.parseLong(next());
        }

        double nextDouble() {
            return Double.parseDouble(next());
        }

        String nextLine() {
            String str = "";
            try {
                str = br.readLine().trim();
            } catch (Exception e) {
                e.printStackTrace();
            }
            return str;
        }
    }
}