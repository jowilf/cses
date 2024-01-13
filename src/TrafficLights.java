import java.io.DataInputStream;
import java.io.IOException;
import java.io.PrintWriter;
import java.util.TreeMap;
import java.util.TreeSet;

/**
 * TrafficLights
 */
public class TrafficLights {

    public static void main(String[] args) {
        FastReader sc = new FastReader();
        PrintWriter out = new PrintWriter(System.out);
        int x = sc.nextInt();
        int n = sc.nextInt();

        TreeMap<Integer, Integer> lengths = new TreeMap<>();
        TreeSet<Integer> lights = new TreeSet<>();

        StringBuilder sb = new StringBuilder();

        lights.add(0);
        lights.add(x);

        lengths.put(x, 1);

        for (int i = 0; i < n; i++) {
            int pos = sc.nextInt();
            // System.out.printf("%d %d %d", lights.last(), lights.first(), pos);
            int lower = pos > 0 ? lights.lower(pos) : 0;
            int high = pos < x ? lights.higher(pos) : x;
            int[] newLengths = new int[] { pos - lower, high - pos };
            for (int l : newLengths) {
                Integer entry = lengths.get(l);
                lengths.put(l, (entry == null ? 0 : entry) + 1);
            }
            int oldLength = high - lower;
            Integer entry = lengths.get(oldLength);
            if (entry == 1) {
                lengths.remove(oldLength);
            } else if (entry != null)
                lengths.put(oldLength, entry - 1);
            lights.add(pos);
            // sb.append(lengths.lastKey() + " ");
            // System.out.println(lengths.lastKey());
            // System.out.println(sb);
            out.print(lengths.lastKey() + " ");
        }
        out.flush();
        // System.out.println(sb);
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