import sys
from queue import Queue

input = sys.stdin.readline

N, M = map(int,input().split())

adj = [[] for _ in range(N+1)]
degree = [0] * (N+1)
queue = Queue()

for i in range(0,M):
    
    data = list(map(int, input().split()))
    input_len = data[0]
    input_list = []

    for j in range(1,input_len + 1):
        input_list.append(data[j])

    for j in range(0,len(input_list)-1):
        adj[input_list[j]].append(input_list[j+1])
        degree[input_list[j+1]] = degree[input_list[j+1]] + 1

for i in range(1,N+1):
    if (degree[i] == 0):
        queue.put(i)

result_list = []

while not queue.empty():
    cur = queue.get()
    result_list.append(cur)

    for i in range(0,len(adj[cur])):
        next = adj[cur][i]
        degree[next] = degree[next] - 1

        if (degree[next] == 0):
            queue.put(next)


if (len(result_list) == N):
    for i in range(0,len(result_list)):
        print(result_list[i])
else:
    print(0)


