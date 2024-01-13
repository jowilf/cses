package dynamic_programming;

import java.io.DataInputStream;
import java.io.IOException;

public class ArrayDescription {
    public static void main(String[] args) {
        FastReader sc = new FastReader();
        int n = sc.nextInt(), m = sc.nextInt();
        long[][] dp = new long[n][m + 1];
        int value = sc.nextInt();
        for (int k = 1; k <= m; k++) {
            dp[0][k] = 0;
            if (value == k || value == 0)
                dp[0][k] = 1;
        }
        for (int i = 1; i < n; i++) {
            value = sc.nextInt();
            for (int k = 1; k <= m; k++) {
                dp[i][k] = 0;
                if (value == 0 || value == k) {
                    dp[i][k] += dp[i - 1][k];
                    if (k > 1)
                        dp[i][k] += dp[i - 1][k - 1];
                    if (k < m)
                        dp[i][k] += dp[i - 1][k + 1];
                    dp[i][k] %= 1000000007;
                }
            }
        }
        // for (Integer[] arr : dp) {
        // System.out.println(Arrays.toString(arr));
        // }
        long ans = 0;
        for (int k = 1; k <= m; k++) {
            ans += dp[n - 1][k];
            ans %= 1000000007;
        }
        System.out.println(ans);

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
