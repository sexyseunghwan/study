package BigSquare1915;

import java.io.*;
import java.util.*;

public class BigSquare1915p {
    static int N,M;
    static int[][] map;
    public static void main(String[] args) throws Exception {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));

        StringTokenizer stk = new StringTokenizer(br.readLine(), " ");

        N = Integer.parseInt(stk.nextToken());
        M = Integer.parseInt(stk.nextToken());
        
        map = new int[N][M];

        for (int i = 0; i < N; i++) {
            String[] inputs = br.readLine().split("");

            for (int j = 0; j < M; j++) {
                map[i][j] = Integer.parseInt(inputs[j]);
            }
        }   

        int maxCnt = 0;

        for (int i = 0; i < N; i++) {
            for (int j = 0; j < M; j++) {
                if (map[i][j] != 0) {
                    
                    int im = i - 1;
                    int jm = j - 1;

                    int mapImJm = (im < 0 || jm < 0) ? 0 : map[i-1][j-1];
                    int mapImJ = (im < 0) ? 0 : map[i-1][j];
                    int mapIJm = (jm < 0) ? 0 : map[i][j-1];
                    
                    map[i][j] = Math.min(mapImJm, Math.min(mapImJ, mapIJm)) + 1;
                    maxCnt = Math.max(maxCnt, map[i][j]);
                }
            }
        }

        bw.write(maxCnt*maxCnt + "");
        bw.close();
        br.close();
    }
}
