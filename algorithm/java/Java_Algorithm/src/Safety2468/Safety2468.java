package Safety2468;

import java.io.*;
import java.util.*;

public class Safety2468 {
    
    static int N;
    static int[][] map;
    static int[][] visited;
    static int maxCnt = 0;
    static int maxHeight;
    static int[] dr = {1,0,-1,0};
    static int[] dc = {0,1,0,-1};
    
    public static void main(String[] args) throws Exception {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));
        
        N = Integer.parseInt(br.readLine());
        
        map = new int[N][N];
        visited = new int[N][N];
        
        for (int i = 0; i < N; i++) {
            StringTokenizer stk = new StringTokenizer(br.readLine(), " ");
            int rowIndex = 0;
            
            while(stk.hasMoreTokens()){
                int height = Integer.parseInt(stk.nextToken());
                map[i][rowIndex++] = height;
                maxHeight = Math.max(height, maxHeight);
            }
        }
        
        for (int i = 0; i <= maxHeight; i++) {
            
            int cnt = 0;
            
            for (int j = 0; j < N*N; j++) {
                int row = j / N;
                int col = j % N;
 
                if (map[row][col] > i && visited[row][col] == 0) { 
                    dfs(row, col, i);
                    cnt++;                                  
                }
            }

            maxCnt = Math.max(maxCnt, cnt);
            visited = new int[N][N];
        }

        bw.write(maxCnt + "");
        bw.close();
        br.close();
    }
    
    static void dfs(int row, int col, int height) {
        
        visited[row][col] = 1;
        
        for (int i = 0; i < 4; i++) {
            int newRow = row + dr[i];
            int newCol = col + dc[i];
            
            if (newRow >= 0 && newCol >= 0 && newRow < N && newCol < N && map[newRow][newCol] > height && visited[newRow][newCol] == 0) {
                dfs(newRow, newCol, height);
            }
        }
    }
}