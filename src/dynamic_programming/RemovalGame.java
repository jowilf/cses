package dynamic_programming;

import java.io.DataInputStream;
import java.io.IOException;
import java.util.Scanner;

public class RemovalGame {
    static class Pair {
        int first;
        int second;

        public Pair(int first, int second) {
            this.first = first;
            this.second = second;
        }

        public int getFirst() {
            return first;
        }

        public void setFirst(int first) {
            this.first = first;
        }

        public int getSecond() {
            return second;
        }

        public void setSecond(int second) {
            this.second = second;
        }
    }

    public static void main(String[] args) {
        FastReader sc = new FastReader();
        int n = sc.nextInt();
        int[] arr = new int[n];
        for (int i = 0; i < arr.length; i++) {
            arr[i] = sc.nextInt();
        }
        Pair[][] dp = new Pair[n][n];
        for (int i = 0; i < n; i++)
            dp[i][i] = new Pair(arr[i], 0);
        for (int l = n - 1; l >= 0; l--) {
            for (int r = l; r < n; r++) {
                if (l != r) {
                    Pair s1 = dp[l + 1][r];// getScore(arr, l + 1, r, dp);
                    Pair s2 = dp[l][r - 1];// getScore(arr, l, r - 1, dp);
                    if (arr[l] + s1.second > arr[r] + s2.second)
                        dp[l][r] = new Pair(arr[l] + s1.second, s1.first);
                    else
                        dp[l][r] = new Pair(arr[r] + s2.second, s2.first);
                }
            }
        }
        System.out.println(dp[0][n - 1].first);
    }

    public static Pair getScore(int[] arr, int l, int r, Pair[][] dp) {
        if (l == r)
            return new Pair(arr[l], 0);
        if (dp[l][r] != null)
            return dp[l][r];
        Pair s1 = getScore(arr, l + 1, r, dp);
        Pair s2 = getScore(arr, l, r - 1, dp);
        if (arr[l] + s1.second > arr[r] + s2.second)
            dp[l][r] = new Pair(arr[l] + s1.second, s1.first);
        else
            dp[l][r] = new Pair(arr[r] + s2.second, s2.first);
        return dp[l][r];
    }

    static class FastReader {
        final private int BUFFER_SIZE = 1 << 16;
        private DataInputStream din;
        private byte[] buffer;
        private int bufferPointer, bytesRead;

        public FastReader() {
            din = new DataInputStream(System.in);
            buffer = new byte[BUFFER_SIZE];
            bufferPointer = bytesRead = 0;
        }

        private void fillBuffer() throws IOException {
            bytesRead = din.read(buffer, bufferPointer = 0, BUFFER_SIZE);
            if (bytesRead == -1)
                buffer[0] = -1;
        }

        private byte read() throws IOException {
            if (bufferPointer == bytesRead)
                fillBuffer();
            return buffer[bufferPointer++];
        }

        public void close() throws IOException {
            if (din == null)
                return;
            din.close();
        }

        public int nextInt() {
            try {
                int ret = 0;
                byte c = read();
                while (c <= ' ')
                    c = read();
                boolean neg = (c == '-');
                if (neg)
                    c = read();
                do {
                    ret = ret * 10 + c - '0';
                } while ((c = read()) >= '0' && c <= '9');

                if (neg)
                    return -ret;
                return ret;
            } catch (IOException e) {
                e.printStackTrace();
                return -1;
            }
        }
    }

}
