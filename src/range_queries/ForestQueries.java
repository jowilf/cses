package range_queries;

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.util.StringTokenizer;

public class ForestQueries {
    public static void main(String[] args) throws IOException {
        BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));
        BufferedWriter writer = new BufferedWriter(new OutputStreamWriter(System.out));

        StringTokenizer tokenizer = new StringTokenizer(reader.readLine());
        int n = Integer.parseInt(tokenizer.nextToken());
        int q = Integer.parseInt(tokenizer.nextToken());

        int[][] grid = new int[n][n];
        int[][] acc = new int[n][n];

        for (int i = 0; i < n; i++) {
            char[] line = reader.readLine().toCharArray();
            for (int j = 0; j < n; j++) {
                grid[i][j] = line[j] == '*' ? 1 : 0;
            }
        }

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                acc[i][j] = grid[i][j];
                if (i - 1 >= 0) {
                    acc[i][j] += acc[i - 1][j];
                }
                if (j - 1 >= 0) {
                    acc[i][j] += acc[i][j - 1];
                }
                if (i - 1 >= 0 && j - 1 >= 0) {
                    acc[i][j] -= acc[i - 1][j - 1];
                }
            }
        }

        for (int k = 0; k < q; k++) {
            tokenizer = new StringTokenizer(reader.readLine());
            int i1 = Integer.parseInt(tokenizer.nextToken()) - 1;
            int j1 = Integer.parseInt(tokenizer.nextToken()) - 1;
            int i2 = Integer.parseInt(tokenizer.nextToken()) - 1;
            int j2 = Integer.parseInt(tokenizer.nextToken()) - 1;

            int ans = acc[i2][j2];
            if (j1 - 1 >= 0) {
                ans -= acc[i2][j1 - 1];
            }
            if (i1 - 1 >= 0) {
                ans -= acc[i1 - 1][j2];
            }
            if (i1 - 1 >= 0 && j1 - 1 >= 0) {
                ans += acc[i1 - 1][j1 - 1];
            }
            writer.write(ans + "\n");
        }

        writer.flush();
        writer.close();
        reader.close();
    }
}
