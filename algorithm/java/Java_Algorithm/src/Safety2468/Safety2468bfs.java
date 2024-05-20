package Safety2468;

import java.io.*;
import java.util.*;

public class Safety2468bfs {

    static int N, maxCnt, minH, maxH;
    static int[][] map;
    static boolean[][] visited;
    static int dr[] = {0,1,0,-1};
    static int dc[] = {1,0,-1,0};

    static class Pair {
        public int row, col;

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
                int height = Integer.parseInt(stk.nextToken());
                maxH = Math.max(height, maxH);
                minH = Math.min(height, minH);

                map[i][index++] = height;
            }
        }
        
        for (int h = minH-1; h < maxH; h++) {
            
            int cnt = 0;
            
            for (int i = 0; i < N*N; i++) {
                int row = i / N;
                int col = i % N;

                if (map[row][col] > h && !visited[row][col]) {
                    bfs(row, col, h);
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

    static void bfs(int row, int col, int height) {
        visited[row][col] = true;
        Queue<Pair> queue = new LinkedList<>();
        queue.add(new Pair(row, col));

        while(!queue.isEmpty()) {

            int curR = queue.peek().row;
            int curC = queue.peek().col;
            queue.poll();

            for (int i = 0; i < 4; i++) {
                int newR = curR + dr[i];
                int newC = curC + dc[i];
                
                if (newR >= 0 && newC >= 0 && newR < N && newC < N && !visited[newR][newC] && map[newR][newC] > height) {
                    queue.add(new Pair(newR, newC));
                    visited[newR][newC] = true;
                }
            }
        }
    }
}