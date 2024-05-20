package Prerequisite14567;

import java.io.*;
import java.util.*;

public class Prerequisite14567 {
    public static void main(String[] args) throws Exception {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));
        
        StringTokenizer stk = new StringTokenizer(br.readLine(), " ");
        int N = Integer.parseInt(stk.nextToken());
        int M = Integer.parseInt(stk.nextToken());

        List<Integer>[] adjList = new ArrayList[N+1];
        int[] degree = new int[N+1];
        int[] result = new int[N+1];
        
        for (int i = 0; i <= N; i++) adjList[i] = new ArrayList<>();
        
        for (int i = 0; i < M; i++) {
            stk = new StringTokenizer(br.readLine(), " ");
            int pre = Integer.parseInt(stk.nextToken());
            int post = Integer.parseInt(stk.nextToken());
            
            adjList[pre].add(post);
            degree[post]++;
        }
        
        Queue<Integer> queue = new ArrayDeque<>();

        for (int i = 1; i <= N; i++) {
            result[i] = 1;
            if (degree[i] == 0) queue.offer(i);
        }

        while (!queue.isEmpty()) {
            int cur = queue.peek();
            queue.poll();

            for (int i = 0; i < adjList[cur].size(); i++) {
                int next = adjList[cur].get(i);
                degree[next]--;
                
                result[next] = Math.max(result[next], result[cur] + 1);

                if (degree[next] == 0) queue.offer(next);
            }
        }

        for (int i = 1; i <= N; i++) bw.write(result[i] + " ");

        bw.flush();
        bw.close();
        br.close();
    }
}