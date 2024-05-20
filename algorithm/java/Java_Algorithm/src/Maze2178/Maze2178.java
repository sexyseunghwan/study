package Maze2178;

import java.io.*;
import java.util.*;

public class Maze2178 {

    static int N,M;
    static int[][] mazeMap;
    static boolean[][] visitedMap;
    static int[] dr = {0,1,-1,0};
    static int[] dc = {1,0,0,-1};
    static Queue<Pair> queue;

    static class Pair {
        public int first;
        public int second;

        public Pair(int first, int second) {
            this.first = first;
            this.second = second;
        }
    }
    
    public static void main(String[] args) throws Exception {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));
        
        StringTokenizer stk = new StringTokenizer(br.readLine(), " ");
        N = Integer.parseInt(stk.nextToken());
        M = Integer.parseInt(stk.nextToken());
        
        mazeMap = new int[N][M];
        visitedMap = new boolean[N][M];

        for (int i = 0 ; i < N; i++) {
            String[] inputs = br.readLine().split("");

            for (int j = 0; j < inputs.length; j++) {
                mazeMap[i][j] = Integer.parseInt(inputs[j]);
            }
        }
        
        visitedMap[0][0] = true;
        queue = new LinkedList<>();
        queue.add(new Pair(0, 0));
        
        while(!queue.isEmpty()) {
            int curR = queue.peek().first;
            int curC = queue.peek().second;
        
            queue.poll();
        
            if (curR == N-1 && curC == M-1) bw.write(mazeMap[curR][curC] + "");
        
            bfs(curR, curC);
        }

        bw.close();
        br.close();
    }    
    
    static void bfs(int curR, int curC) {
        
        for (int i = 0; i < 4; i++) {
            int newR = curR + dr[i];
            int newC = curC + dc[i];
            
            if (newR >= 0 && newC >= 0 && newR < N && newC < M && !visitedMap[newR][newC] && mazeMap[newR][newC] != 0) {
                visitedMap[newR][newC] = true;
                queue.add(new Pair(newR, newC));
                mazeMap[newR][newC] = mazeMap[curR][curC] + 1;
            }
        }
    }
}