import java.io.DataInputStream;
import java.io.IOException;
import java.util.TreeMap;

public class MaximumSubarraySumII {
    public static void main(String[] args) {
        FastReader sc = new FastReader();
        int n = sc.nextInt(), a = sc.nextInt(), b = sc.nextInt();
        long[] arr = new long[n];
        arr[0] = sc.nextInt();
        for (int i = 1; i < n; i++)
            arr[i] = arr[i - 1] + sc.nextInt();
        // System.out.println(Arrays.toString(arr));
        long ans = Long.MIN_VALUE;
        TreeMap<Long, Integer> tree = new TreeMap<>();
        for (int i = 0; i < n; i++) {
            if ((i - a) >= 0) {
                increaseElementValue(tree, arr[i - a]);
                // System.out.println("inc " + arr[i - a]);
            }
            if ((i - b - 1) >= 0) {
                decreaseElementValue(tree, arr[i - b - 1]);
                // System.out.println("dec " + arr[i - b - 1]);
            }
            // System.out.println("i " + i + "; tree " + tree + "; fentry " +
            // tree.firstEntry());
            var entry = tree.firstEntry();
            if (entry != null)
                ans = Math.max(ans, arr[i] - entry.getKey());
            if (a <= (i + 1) && (i + 1) <= b)
                ans = Math.max(ans, arr[i]);
        }
        System.out.println(ans);
    }

    private static void increaseElementValue(TreeMap<Long, Integer> t, long e) {
        t.put(e, t.getOrDefault(e, 0) + 1);
    }

    private static void decreaseElementValue(TreeMap<Long, Integer> t, long e) {
        Integer v = t.get(e);
        if (v != null && v == 1)
            t.remove(e);
        else
            t.put(e, v - 1);
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
