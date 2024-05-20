package WorkBook1766;

import java.io.*;
import java.util.*;

public class WorkBook1766 {

    public static void main(String[] args) throws Exception {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));

        StringTokenizer stk = new StringTokenizer(br.readLine()," ");
        int N = Integer.parseInt(stk.nextToken());
        int M = Integer.parseInt(stk.nextToken());

        int[] degree = new int[N + 1];
        List<Integer>[] adj = new ArrayList[N+1];

        for (int i = 0; i <= N; i++) {
            adj[i] = new ArrayList<>();
        }

        PriorityQueue<Integer> pq = new PriorityQueue<>(new Comparator<Integer>() {
            @Override
            public int compare(Integer o1, Integer o2) {
                if (o1 > o2) return 1;
				else if (o2 > o1) return -1;
				else return 0;
            }
		}); 
        
        for (int i = 0; i < M; i++) {
            stk = new StringTokenizer(br.readLine()," ");
            int pre = Integer.parseInt(stk.nextToken());
            int post = Integer.parseInt(stk.nextToken());
            
            adj[pre].add(post);
            degree[post]++;
        }
ㄴ
        for (int i = 1; i <= N; i++) {
            if (degree[i] == 0) pq.offer(i);
        }

        while (!pq.isEmpty()) {
            int cur = pq.peek();
            pq.poll();

            bw.write(cur + " ");

            for (int i = 0; i < adj[cur].size(); i++) {
                int next = adj[cur].get(i);
                degree[next]--;
                
                if (degree[next] == 0) pq.offer(next);
            }
        }

        bw.flush();
        bw.close();
        br.close();
    }
}