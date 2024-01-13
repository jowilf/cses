import java.util.TreeMap;
import java.io.DataInputStream;
import java.io.IOException;

/**
 * Towers
 */
public class Towers {
    public static void main(String[] args) {
        FastReader sc = new FastReader();
        int n = sc.nextInt();
        TreeMap<Integer, Integer> towers = new TreeMap<>();
        towers.put(sc.nextInt(), 1);
        int ans = 1;
        for (int i = 1; i < n; i++) {
            int value = sc.nextInt();
            var entry = towers.higherEntry(value);
            if (entry == null) {
                ans += 1;
            } else {
                if (entry.getValue() > 1) {
                    towers.put(entry.getKey(), entry.getValue() - 1);
                } else
                    towers.remove(entry.getKey());
            }
            Integer existingValue = towers.get(value);
            if (existingValue == null)
                towers.put(value, 1);
            else
                towers.put(value, existingValue + 1);
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