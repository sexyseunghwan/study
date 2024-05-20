import sys

# 재귀 호출 깊이 제한 설정
sys.setrecursionlimit(100000) 

input = sys.stdin.readline

dr = [0,1,0,-1]
dc = [1,0,-1,0]

N = int(input())

max_cnt = 0
min_h = 0
max_h = 0
visited = [[0] * N for _ in range(N)]
safety_map = []


def dfs(row,col,h):
    visited[row][col] = 1
    
    for i in range(4):
        new_r = row + dr[i]
        new_c = col + dc[i]

        if (new_r >= 0 and new_c >= 0 and new_r < N and new_c < N and visited[new_r][new_c] == 0 and safety_map[new_r][new_c] > h):
            dfs(new_r, new_c, h)
    

for i in range(N):
    numbers = list(map(int, input().split()))
    
    for j in range(len(numbers)):
        min_h = min(numbers[j], min_h)
        max_h = max(numbers[j], max_h)
    
    safety_map.append(numbers)



for h in range(min_h-1, max_h):
    
    cur_cnt = 0

    for i in range(N*N):
        row = i // N
        col = i % N

        if (safety_map[row][col] > h and visited[row][col] == 0):
            dfs(row,col,h)
            cur_cnt += 1

    visited = [[0] * N for _ in range(N)]
    max_cnt = max(max_cnt, cur_cnt)

print(max_cnt)

