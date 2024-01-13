import java.io.DataInputStream;
import java.io.IOException;
import java.util.TreeMap;
import java.util.Map.Entry;

public class ConcertTickets {
    public static void main(String[] args) {
        FastReader sc = new FastReader();
        int n = sc.nextInt(), m = sc.nextInt();
        TreeMap<Integer, Integer> pricesMap = new TreeMap<>();
        for (int i = 0; i < n; i++) {
            int h = sc.nextInt();
            pricesMap.put(h, pricesMap.getOrDefault(h, 0) + 1);
        }
        StringBuilder stb = new StringBuilder();
        for (int i = 0; i < m; i++) {
            int t = sc.nextInt();
            Entry<Integer, Integer> price = pricesMap.lowerEntry(t + 1);
            if (price == null)
                stb.append(-1 + "\n");
            else {
                stb.append(price.getKey() + "\n");
                if (price.getValue() > 1)
                    pricesMap.put(price.getKey(), price.getValue() - 1);
                else
                    pricesMap.remove(price.getKey());
            }
        }
        System.out.print(stb);
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