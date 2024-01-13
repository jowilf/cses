import java.io.DataInputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Arrays;
import java.util.Comparator;
import java.util.TreeSet;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class NearestSmallerValues {
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

        public int getSecond() {
            return second;
        }

    }

    public static void main(String[] args) {
        FastReader sc = new FastReader();
        int n = sc.nextInt();

        int[] ans = new int[n];
        Arrays.fill(ans, 0);
        TreeSet<Integer> indexes = new TreeSet<>();
        List<Pair> values = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            values.add(new Pair(sc.nextInt(), i));
        }
        values.sort(Comparator.comparing(Pair::getFirst).thenComparing(p -> p.getSecond() * -1));

        for (Pair v : values) {
            var entry = indexes.lower(v.second);
            // System.out.printf("%s ; %s", v.first, indexes);
            if (entry != null) {
                ans[v.second] = entry.intValue() + 1;
            }
            indexes.add(v.second);
        }
        System.out.println(IntStream.of(ans)
                .mapToObj(String::valueOf).collect(Collectors.joining(" ")));
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
