import sys
from queue import Queue

input = sys.stdin.readline

N, M = map(int,input().split())

adj = [[] for _ in range(N+1)]
degree = [0] * (N+1)
queue = Queue()

for i in range(0,M):
    pre, post = map(int,input().split())

    adj[pre].append(post)
    degree[post] = degree[post] + 1

for i in range(1,N+1):
    if (degree[i] == 0):
        queue.put(i)

while not queue.empty():

    cur = queue.get()
    
    print(cur)
    
    for i in range(0,len(adj[cur])):
        next = adj[cur][i]
        degree[next] = degree[next] - 1

        if (degree[next] == 0):
            queue.put(next)

