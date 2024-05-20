import sys
from queue import Queue

input = sys.stdin.readline

N, M = map(int,input().split())

adj = [[] for _ in range(N+1)]
degree = [0] * (N+1)
result_list = [1] * (N+1)
queue = Queue()

for i in range(M):
    pre, post = map(int,input().split())

    adj[pre].append(post)   
    degree[post] += 1

for i in range(1,N+1):
    if (degree[i] == 0):
        queue.put(i)

while not queue.empty():
    cur = queue.get()

    for next in adj[cur]:
        degree[next] -= 1
        result_list[next] = max(result_list[next], result_list[cur] + 1)

        if (degree[next] == 0):
            queue.put(next)

for i in range(1,N+1):
    print(result_list[i])