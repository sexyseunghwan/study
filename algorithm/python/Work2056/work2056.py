import sys
from queue import Queue

input = sys.stdin.readline

N = int(input())

adj = [[] for _ in range(N+1)]

times = [0] * (N+1)
result = [0] * (N+1)
degree = [0] * (N+1)
answer = 0

for i in range(1,N+1):
    inputs_list = list(map(int, input().split()))
    
    times[i] = inputs_list[0]
    result[i] = inputs_list[0]

    inner_pres = inputs_list[1]

    for j in range(0,inner_pres):
        adj[inputs_list[j+2]].append(i)
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

for i in range(1, N+1):
    answer = max(answer,result[i])


print(answer)