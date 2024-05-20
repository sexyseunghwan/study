package Laboratory14502;

import java.io.*;
import java.util.*;

public class Laboratory14502 {

    static int N,M;
    static int maxSafetyArea;
    static int[][] map;
    static int[] dr = {0,0,1,-1};
    static int[] dc = {1,-1,0,0};
    static List<Pair> virusList;

    static class Pair {
        int r;
        int c;

        public Pair(int r, int c) {
            this.r = r;
            this.c = c;
        }
    }

    public static void main(String[] args) throws Exception {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));

        StringTokenizer stk = new StringTokenizer(br.readLine(), " ");

        N = Integer.parseInt(stk.nextToken());
        M = Integer.parseInt(stk.nextToken());

        virusList = new ArrayList<>();
        map = new int[N][M];
        
        for (int i = 0; i < N; i++) {
            String[] inputs = br.readLine().split(" ");
            for (int j = 0; j < inputs.length; j++) {
                int inputNum = Integer.parseInt(inputs[j]);
                
                if (inputNum == 2) virusList.add(new Pair(i, j));
                
                map[i][j] = Integer.parseInt(inputs[j]);
            }
        }   

        dfs(0,0);

        bw.write(maxSafetyArea + "");
        
        bw.flush();
        bw.close();
        br.close();
    }

    static void dfs(int index, int cnt) {
        
        if (cnt == 3){
            bfs();
            return;
        }
        
        for (int i = index; i < N*M; i++) {
            int r = i/M;
            int c = i%M;

            if (map[r][c] == 0) {
                map[r][c] = 1;
                dfs(i+1,cnt + 1);
                map[r][c] = 0;        
            }
        }
    }

    static void bfs() {

        int[][] virusMap = new int[N][M];
        for (int i = 0; i < N*M; i++) virusMap[i/M][i%M] = map[i/M][i%M];
        
        Queue<Pair> que = new ArrayDeque<>();
        
        for (Pair pairs : virusList) {
            que.offer(pairs);
        }

        while(!que.isEmpty()) {
            Pair pair = que.peek();
            que.poll();

            for (int i = 0; i < 4; i++) {
                int newR = pair.r + dr[i];
                int newC = pair.c + dc[i];
            
                if (newR >= 0 && newC >= 0 && newR < N && newC < M && virusMap[newR][newC] == 0) {
                    que.offer(new Pair(newR, newC));
                    virusMap[newR][newC] = 2;
                }
            }
        }

        int safetyArea = 0;

        for (int i = 0; i < M*N; i++) 
            if (virusMap[i/M][i%M] == 0) safetyArea++;
        
        
        maxSafetyArea = Math.max(maxSafetyArea, safetyArea);
    }
}