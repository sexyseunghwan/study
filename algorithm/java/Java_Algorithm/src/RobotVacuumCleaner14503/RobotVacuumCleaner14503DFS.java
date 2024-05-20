package RobotVacuumCleaner14503;

import java.io.*;
import java.util.*;

public class RobotVacuumCleaner14503DFS {
    
    static int N,M,R,C,V;
    static int[] dr = {-1,0,1,0};    
    static int[] dc = {0,1,0,-1};
    static int[][] map;
    static int cleanCnt;

    public static void main(String[] args) throws Exception {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));
        
        StringTokenizer stk = new StringTokenizer(br.readLine(), " ");
        N = Integer.parseInt(stk.nextToken());        
        M = Integer.parseInt(stk.nextToken());

        map = new int[N][M];
        
        stk = new StringTokenizer(br.readLine(), " ");
        R = Integer.parseInt(stk.nextToken());        
        C = Integer.parseInt(stk.nextToken());
        V = Integer.parseInt(stk.nextToken());

        for (int i = 0; i < N; i++) {
            stk = new StringTokenizer(br.readLine(), " ");

            for (int j = 0; j < M; j++) map[i][j] = Integer.parseInt(stk.nextToken());
        }
        
        dfs(R,C,V,1);

        bw.write(cleanCnt + "");
        bw.flush();
        bw.close();
        br.close();
    }

    static void dfs(int r, int c, int v, int p) {
        
        if (p == 1) {
            map[r][c] = 2;
            cleanCnt++;
        }

        boolean flag = false;

        for (int i = 0; i < 4; i++) {
            int newR = r + dr[i];
            int newC = c + dc[i];

            if (map[newR][newC] == 0)
            {
                flag = true;
                break;
            }
        }

        if (flag) {
            int frontV = leftTurn(v);

            int frontR = r + dr[frontV];
            int frontC = c + dc[frontV];

            if (map[frontR][frontC] == 0) dfs(frontR, frontC, frontV, 1);
            else dfs(r,c,frontV,0);

        } else {

            int backV = backTurn(v);

            int backR = r + dr[backV];
            int backC = c + dc[backV];
            
            if (map[backR][backC] != 1) dfs(backR, backC, v, 0); 
        }

        return;
    }
    
    static int backTurn(int v) {
        int newV = (v + 2) % 4;
        return newV; 
    }

    static int leftTurn(int v)
    {
        int newV = (v + 3) % 4;
        return newV;
    }
}
