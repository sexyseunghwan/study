package RobotVacuumCleaner14503;

import java.io.*;
import java.util.*;

public class RobotVacuumCleaner14503StackModify {
    
    static int N,M,R,C,V;
    static int[] dr = {-1,0,1,0};    
    static int[] dc = {0,1,0,-1};
    static int[][] map;
    static int cleanCnt;

    static class Elem {
        public int r,c,v,p;

        public Elem(int r, int c, int v, int p) {
            this.r = r;
            this.c = c;
            this.v = v;
            this.p = p;
        }
    }

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
        
        Stack<Elem> stack = new Stack<>();
        stack.push(new Elem(r,c,v,p));

        while(!stack.isEmpty()) {
            
            Elem elem = stack.peek();
            stack.pop();
            
            if (elem.p == 1) {
                map[elem.r][elem.c] = 2;
                cleanCnt++;
            }
    
            boolean flag = false;
    
            for (int i = 0; i < 4; i++) {
                int newR = elem.r + dr[i];
                int newC = elem.c + dc[i];
    
                if (map[newR][newC] == 0)
                {
                    flag = true;
                    break;
                }
            }
    
            if (flag) {
                int frontV = leftTurn(elem.v);
    
                int frontR = elem.r + dr[frontV];
                int frontC = elem.c + dc[frontV];
    
                if (map[frontR][frontC] == 0) stack.push(new Elem(frontR, frontC, frontV, 1));
                else stack.push(new Elem(elem.r, elem.c, frontV, 0));
    
            } else {
    
                int backV = backTurn(elem.v);
    
                int backR = elem.r + dr[backV];
                int backC = elem.c + dc[backV];
                
                if (map[backR][backC] != 1) stack.push(new Elem(backR, backC, elem.v, 0));
            }        
        }
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
