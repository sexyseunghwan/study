package BigSquare1915;

import java.io.*;
import java.util.*;

public class BigSquare1915 {
    static int N,M;
    static int[][] map;
    public static void main(String[] args) throws Exception {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));

        StringTokenizer stk = new StringTokenizer(br.readLine(), " ");

        N = Integer.parseInt(stk.nextToken());
        M = Integer.parseInt(stk.nextToken());
        
        map = new int[N+1][M+1];

        for (int i = 1; i <= N; i++) {
            String[] inputs = br.readLine().split("");

            for (int j = 0; j < M; j++) {
                map[i][j+1] = Integer.parseInt(inputs[j]);
            }
        }   

        int maxCnt = 0;

        for (int i = 1; i <= N; i++) {
            for (int j = 1; j <= M; j++) {
                if (map[i][j] != 0) {
                    map[i][j] = Math.min(map[i-1][j-1], Math.min(map[i-1][j], map[i][j-1])) + 1;
                    maxCnt = Math.max(maxCnt, map[i][j]);
                }
            }
        }

        bw.write(maxCnt*maxCnt + "");
        bw.close();
        br.close();
    }
}
