package Safety2468;

import java.io.*;
import java.util.*;

public class Safety2468stack {
    
    static int N, minH, maxH, maxCnt;
    static int[][] map;
    static boolean[][] visited;
    static int dr[] = {0,1,0,-1};
    static int dc[] = {1,0,-1,0};
    
    static class Pair {
        int row,col;

        public Pair(int row, int col) {
            this.row = row;
            this.col = col;
        }
    }

    public static void main(String[] args) throws Exception {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));

        N = Integer.parseInt(br.readLine());
        map = new int[N][N];
        visited = new boolean[N][N];

        for (int i = 0; i < N; i++) {
            StringTokenizer stk = new StringTokenizer(br.readLine(), " ");
            int index = 0;

            while(stk.hasMoreTokens()) {
                int curH = Integer.parseInt(stk.nextToken());
                minH = Math.min(minH, curH);
                maxH = Math.max(maxH, curH);

                map[i][index++] = curH;
            }
        }
        
        for (int h = minH-1; h < maxH; h++) {
            
            int cnt = 0;
            
            for (int i = 0; i < N*N; i++) {
                int curR = i / N;
                int curC = i % N;

                if (map[curR][curC] > h && !visited[curR][curC]) {
                    dfs(curR, curC, h);
                    cnt++;
                }
            }

            maxCnt = Math.max(maxCnt, cnt);
            visited = new boolean[N][N];
        }
        
        bw.write(maxCnt + "");
        bw.close();
        br.close();
    }

    static void dfs(int row, int col, int height) {
        
        Stack<Pair> stack = new Stack<>();
        stack.push(new Pair(row, col));
        visited[row][col] = true;

        while(!stack.isEmpty()) {
            int curR = stack.peek().row;
            int curC = stack.peek().col;
            stack.pop();

            for (int i = 0; i < 4; i++) {
                int newR = curR + dr[i];
                int newC = curC + dc[i];

                if (newR >= 0 && newC >= 0 && newR < N && newC < N && !visited[newR][newC] && map[newR][newC] > height) {
                    stack.push(new Pair(newR, newC));
                    visited[newR][newC] = true;
                }
            }
        }
    }
}



