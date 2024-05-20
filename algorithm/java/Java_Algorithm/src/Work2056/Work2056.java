package Work2056;

import java.io.*;
import java.util.*;

public class Work2056 {
    public static void main(String[] args) throws Exception {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));

        int N = Integer.parseInt(br.readLine());
        
        int[] degree = new int[N+1];
        int[] times = new int[N+1];
        int[] result = new int[N+1];
        List<Integer>[] adj = new ArrayList[N+1];
        Queue<Integer> queue = new LinkedList<>();
        int answer = 0;

        for (int i = 0; i <= N; i++) adj[i] = new ArrayList<>();
        
        for (int i = 1; i <= N; i++) {
            StringTokenizer stk = new StringTokenizer(br.readLine(), " ");

            int useTime = Integer.parseInt(stk.nextToken());
            int preCnt = Integer.parseInt(stk.nextToken());

            times[i] = useTime;
            result[i] = useTime;
            
            if (preCnt == 0) continue;

            while(stk.hasMoreTokens()) {
                int preNode = Integer.parseInt(stk.nextToken());
                
                adj[preNode].add(i);
                degree[i]++;
            }   
        }
        
        for (int i = 1; i <= N; i++) {
            if (degree[i] == 0) queue.offer(i);
        }

        while(!queue.isEmpty()) {

            int cur = queue.peek();
            queue.poll();

            for (int i = 0; i < adj[cur].size(); i++) {
                int next = adj[cur].get(i);
                result[next] = Math.max(result[next], result[cur] + times[next]);
                degree[next]--;

                if (degree[next] == 0) queue.offer(next);
            }
        }
        
        for (int res : result) {
            answer = Math.max(res, answer);
        }


        bw.write(answer + "");
        bw.flush();
        bw.close();
        br.close();
    }
}