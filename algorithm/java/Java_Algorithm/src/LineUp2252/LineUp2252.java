package LineUp2252;

import java.io.*;
import java.util.*;

public class LineUp2252 {
    
    static int N,M;
    static int[] degreeList;
    static List<Integer>[] adj;
    public static void main(String[] args) throws Exception {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));

        StringTokenizer stk = new StringTokenizer(br.readLine(), " ");
        
        N = Integer.parseInt(stk.nextToken());
        M = Integer.parseInt(stk.nextToken());
        
        degreeList = new int[N + 1];
        adj = new ArrayList[N + 1];

        for (int i = 0; i <= N; i++) adj[i] = new ArrayList<>();
        
        for (int i = 0; i < M; i++) {
            stk = new StringTokenizer(br.readLine(), " ");

            int pre = Integer.parseInt(stk.nextToken());
            int post = Integer.parseInt(stk.nextToken());

            adj[pre].add(post);
            degreeList[post]++;
        }
        
        Queue<Integer> que = new ArrayDeque<>();

        for (int i = 1; i <= N; i++) {
            if (degreeList[i] == 0) que.offer(i);
        }

        while(!que.isEmpty()) {
            int cur = que.peek();
            que.poll();
            
            bw.write(cur + " ");
            
            for (int i = 0; i < adj[cur].size(); i++) {
                int next = adj[cur].get(i);
                degreeList[next]--;

                if (degreeList[next] == 0) que.offer(next);
            }
        }

        bw.flush();
        bw.close();
        br.close();
    }
}
