package Maze2178;

import java.io.*;
import java.util.*;

public class Maze2178bfs {

    static int N,M;
    static int[][] maze;
    static boolean[][] visited;
    static int[] dr = {1,0,-1,0};
    static int[] dc = {0,1,0,-1};

    static class Pair {
        public int row;
        public int col;

        public Pair(int row, int col) {
            this.row = row;
            this.col = col;
        }
    }   

    public static void main(String[] args) throws Exception {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));

        StringTokenizer stk = new StringTokenizer(br.readLine(), " ");
        N = Integer.parseInt(stk.nextToken());
        M = Integer.parseInt(stk.nextToken());
        
        maze = new int[N][M];
        visited = new boolean[N][M];

        for (int i = 0; i < N; i++) {
            String[] inputs = br.readLine().split("");
            for (int j = 0; j < M; j++) {
                maze[i][j] = Integer.parseInt(inputs[j]);
            }
        }

        bfs(0,0);   

        bw.write(maze[N-1][M-1] + "");
        bw.close();
        br.close();
    }

    static void bfs(int row, int col) {
        
        visited[row][col] = true;
        Queue<Pair> queue = new LinkedList<>();
        queue.add(new Pair(row, col));

        while (!queue.isEmpty()) {
            int curR = queue.peek().row;
            int curC = queue.peek().col;

            queue.poll();

            if (curR == N-1 && curC == M-1) break;   

            for (int i = 0; i < 4; i++) {
                int newR = curR + dr[i];
                int newC = curC + dc[i];
                
                if (newR >= 0 && newC >= 0 && newR < N && newC < M && !visited[newR][newC] && maze[newR][newC] != 0) {
                    visited[newR][newC] = true;
                    queue.add(new Pair(newR, newC));
                    maze[newR][newC] = maze[curR][curC] + 1;
                }
            }
        }
    }
}