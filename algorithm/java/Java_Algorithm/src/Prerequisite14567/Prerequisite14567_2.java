package Prerequisite14567;

import java.io.*;
import java.util.*;

public class Prerequisite14567_2 {
    public static void main(String[] args) throws Exception {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));
        
        StringTokenizer stk = new StringTokenizer(br.readLine(), " ");
        
        int N = Integer.parseInt(stk.nextToken());
        int M = Integer.parseInt(stk.nextToken());

        int[] resultList = new int[N+1];

        Map<Integer, List<Integer>> adjList = new HashMap<>();
        int[] degreeList = new int[N+1];

        for (int i = 0; i <= N; i++) adjList.put(i, new ArrayList<>());
        
        for (int i = 0; i < M; i++) {
            stk = new StringTokenizer(br.readLine(), " ");

            int pre = Integer.parseInt(stk.nextToken());
            int post = Integer.parseInt(stk.nextToken());
            
            adjList.get(pre).add(post);
            degreeList[post]++;
        }
        
        Queue<Integer> que = new ArrayDeque<>();

        for (int i = 1; i <= N; i++) {
            resultList[i] = 1;
            if (degreeList[i] == 0) que.offer(i);
        }

        while(!que.isEmpty()) {
            
            int cur = que.peek();
            que.poll();

            for (int i = 0; i < adjList.get(cur).size(); i++) {
                int next = adjList.get(cur).get(i);
                degreeList[next]--;

                resultList[next] = Math.max(resultList[next], resultList[cur] + 1);

                if (degreeList[next] == 0) que.offer(next);
            }
        }
        
        for (int i = 1; i <= N; i++) bw.write(resultList[i] + " ");

        bw.flush();
        bw.close();
        br.close();
        
    }
}