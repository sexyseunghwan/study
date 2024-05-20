package MusicProgram2623;

import java.io.*;
import java.util.*;

public class MusicProgram2623 {
    public static void main(String[] args) throws Exception {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));
        
        StringTokenizer stk = new StringTokenizer(br.readLine(), " ");
        int N = Integer.parseInt(stk.nextToken());
        int M = Integer.parseInt(stk.nextToken());
        
        List<Integer>[] adj = new ArrayList[N+1];
        int degree[] = new int[N+1];
        Queue<Integer> queue = new ArrayDeque<>();

        for (int i = 0; i <= N; i++) adj[i] = new ArrayList<>();
        
        for (int i = 0; i < M; i++) {
            
            stk = new StringTokenizer(br.readLine(), " ");
            int cnt = Integer.parseInt(stk.nextToken());
            int[] inputArr = new int[cnt];
            
            int index = 0;

            while(stk.hasMoreTokens()) {
                inputArr[index++] = Integer.parseInt(stk.nextToken());
            }
            
            for (int j = 0; j < cnt-1; j++) {
                adj[inputArr[j]].add(inputArr[j+1]);
                degree[inputArr[j+1]]++;
            }
        }
        
        for (int i = 1; i <= N; i++) {
            if (degree[i] == 0) queue.offer(i);
        }
        
        List<Integer> resultArr = new ArrayList<>();

        while (!queue.isEmpty()) {
            int cur = queue.peek();
            queue.poll();
            resultArr.add(cur);

            for (int i = 0; i < adj[cur].size(); i++) {
                int next = adj[cur].get(i);
                degree[next]--;

                if (degree[next] == 0) queue.offer(next);
            }
        }

        if (resultArr.size() == N) {
            for (int res : resultArr) {
                bw.write(res + "\n");
            }
        } else {
            bw.write(0 + "");
        }

        bw.flush();
        bw.close();
        br.close();
        
    }
}
