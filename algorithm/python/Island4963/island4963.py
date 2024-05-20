import sys
input = sys.stdin.readline

dr = [0,0,1,-1,1,-1,1,-1]
dc = [1,-1,0,0,1,-1,-1,1]

def dfs(r,c):
    stack = [(r,c)]
    island_map[r][c] = 0

    while stack:

        x,y = stack.pop()

        for i in range (0,8):
            nr = x + dr[i]
            nc = y + dc[i]

            if (nr >= 0 and nc >= 0 and nr < N and nc < M and island_map[nr][nc] != 0):
                stack.append((nr,nc))
                island_map[nr][nc] = 0  

while (True):
        
    island_cnt = 0
    M, N = map(int,input().split())

    if (N == M == 0):
        break
    
    island_map = [list(map(int, input().split())) for _ in range(N)]

    for i in range (0,N*M):
        if (island_map[i//M][i%M] != 0):
            dfs(i//M,i%M)
            island_cnt += 1


    print(island_cnt)


