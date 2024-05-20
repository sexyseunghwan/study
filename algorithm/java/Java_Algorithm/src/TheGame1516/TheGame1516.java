package TheGame1516;

import java.io.*;
import java.util.*;

public class TheGame1516 {
    static int N;
    static int[] result;
    static int[] times;
    static int[] degree;
    static List<Integer>[] ver;
    static List<Integer>[] adj;

    public static void main(String[] args) throws Exception {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));

        N = Integer.parseInt(br.readLine());
        result = new int[N+1];
        times = new int[N+1];
        degree = new int[N+1];
        
        ver = new ArrayList[N+1];
        adj = new ArrayList[N+1];

        for (int i = 0; i <= N; i++) {
            ver[i] = new ArrayList<>();
            adj[i] = new ArrayList<>();
        }

        for (int i = 1; i <= N; i++) {
            StringTokenizer stk = new StringTokenizer(br.readLine(), " ");

            while(stk.hasMoreTokens()) {
                int input = Integer.parseInt(stk.nextToken());

                if (input == -1) break;
                else ver[i].add(input);
            }
        }
        
        for (int i = 1; i <= N; i++) times[i] = ver[i].get(0);
        
        for (int i = 1; i <= N; i++) {
            for (int j = 1; j < ver[i].size(); j++) {
                adj[ver[i].get(j)].add(i);
                degree[i]++;
            }
        }

        Queue<Integer> queue = new ArrayDeque<>();

        for (int i = 1; i <= N; i++) {
            if (degree[i] == 0) queue.offer(i);
            result[i] = times[i];
        }
        
        while (!queue.isEmpty()) {
            int cur = queue.peek();
            queue.poll();

            for (int i = 0; i < adj[cur].size(); i++) {
                int next = adj[cur].get(i);
                degree[next]--;
                
                result[next] = Math.max(result[next], result[cur] + times[next]);

                if (degree[next] == 0) queue.offer(next);
            }
        }

        for (int i = 1; i <= N; i++) bw.write(result[i] + "\n");
        
        bw.flush();
        bw.close();
        br.close();

    }

}