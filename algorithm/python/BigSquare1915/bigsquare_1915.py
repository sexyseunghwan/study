import sys
input = sys.stdin.readline

N, M = map(int, input().split())
map = [[0] * (M + 1) for _ in range(N + 1)]

for i in range(1, N + 1):
    inputs = list(input().strip())
    for j in range(M):
        map[i][j + 1] = int(inputs[j])

max_cnt = 0

for i in range(1, N + 1):
    for j in range(1, M + 1):
        if (map[i][j] != 0):
            map[i][j] = min(map[i-1][j-1], min(map[i-1][j], map[i][j-1])) + 1
            max_cnt = max(max_cnt, map[i][j])

print(max_cnt*max_cnt)