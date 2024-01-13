package dynamic_programming;

import java.io.DataInputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Scanner;
import java.util.TreeMap;

public class Projects {

    static class P {
        int a, b, s;

        public P(int a, int b, int s) {
            this.a = a;
            this.b = b;
            this.s = s;
        }
    }

    public static void main(String[] args) {
        FastReader sc = new FastReader();
        int n = sc.nextInt();
        int[] a = new int[n], b = new int[n], s = new int[n];
        int cnt = 0;
        HashMap<Integer, List<P>> map = new HashMap<>();
        HashMap<Integer, Integer> days = new HashMap<>();
        for (int i = 0; i < n; i++) {
            a[i] = sc.nextInt();
            b[i] = sc.nextInt();
            s[i] = sc.nextInt();
            days.put(a[i], 0);
            days.put(b[i], 0);
        }
        // days.keySet().stream().sorted().forEach(t -> days.put(t, cnt++));

        for (int day : days.keySet()) {
            System.out.println(day+";"+days.get(day));
        }
        
        System.out.println();
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
