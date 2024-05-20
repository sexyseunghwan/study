package Island4963;
import java.io.*;
import java.util.*;

public class Island4963_bfs {

    static int N,M;
    static int[][] map;
    static int[][] visitedMap;
    static int[] dr = {0,0,1,-1,1,-1,1,-1};
    static int[] dc = {1,-1,0,0,1,-1,-1,1};

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

        while (true) {
            StringTokenizer stk = new StringTokenizer(br.readLine(), " ");
            M = Integer.parseInt(stk.nextToken());
            N = Integer.parseInt(stk.nextToken());
            
            int islandCnt = 0;

            if (M == 0 && N == 0) break;
            
            map = new int[N][M];
            visitedMap = new int[N][M];

            for (int i = 0; i < N; i++) {
                stk = new StringTokenizer(br.readLine(), " ");
                for (int j = 0; j < M; j++) map[i][j] = Integer.parseInt(stk.nextToken());
            }

            for (int i = 0; i < N*M; i++) {
                int r = i/M;
                int c = i%M;

                if (map[r][c] != 0 && visitedMap[r][c] == 0) {
                    bfs(r, c);
                    islandCnt++;
                }
            }
            
            bw.write(islandCnt + "\n");
        }
        
        bw.close();
        br.close();
    }    
    
    static void bfs(int r, int c){
            
        visitedMap[r][c] = 1;
        Queue<Pair> queue = new LinkedList<>();
        queue.add(new Pair(r,c));
        
        while (!queue.isEmpty()) {
            
            int curR = queue.peek().first;
            int curC = queue.peek().second;
            queue.poll();  
            
            for (int i = 0; i < 8; i++) {
                int newR = curR + dr[i];
                int newC = curC + dc[i];
                
                if (newR >= 0 && newC >= 0 && newR < N && newC < M && map[newR][newC] != 0 && visitedMap[newR][newC] == 0) {
                    bfs(newR, newC);
                    visitedMap[newR][newC] = 1;
                }
            }
        }
    }
}
