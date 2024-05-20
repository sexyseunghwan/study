import sys
from queue import Queue

input = sys.stdin.readline

N = int(input())

adj = [[] for _ in range(N+1)]
vertex = [[] for _ in range(N+1)]

times = [0] * (N+1)
result = [0] * (N+1)
degree = [0] * (N+1)

for i in range(1,N+1):
    data = list(map(int, input().split()))[:-1]
    for j in range(0,len(data)):
        vertex[i].append(data[j])
    
for i in range(1, N+1):
    times[i] = vertex[i][0]
    result[i] = vertex[i][0]

for i in range(1, N+1):
    for j in range(1, len(vertex[i])):
        adj[vertex[i][j]].append(i)
        degree[i] = degree[i] + 1

queue = Queue()

for i in range(1, N+1):
    if (degree[i] == 0):
        queue.put(i)

while not queue.empty():

    cur = queue.get()

    for i in range(0, len(adj[cur])):
        next = adj[cur][i]
        degree[next] = degree[next] - 1

        result[next] = max(result[next], result[cur] + times[next])

        if (degree[next] == 0):
            queue.put(next)

for i in range(1,N+1):
    print(result[i])