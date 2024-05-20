import sys
input = sys.stdin.readline

N, M = map(int, input().split())
map = [[0] * M for _ in range(N)]

for i in range(N):
    inputs = list(input().strip())
    for j in range(M):
        map[i][j] = int(inputs[j])

max_cnt = 0

for i in range(N):
    for j in range(M):
        if (map[i][j] != 0):
            
            map_im_jm = map_im_j = map_i_jm = 0    

            map[i][j] = min(map[i-1][j-1], min(map[i-1][j], map[i][j-1])) + 1
            max_cnt = max(max_cnt, map[i][j])

print(max_cnt*max_cnt)