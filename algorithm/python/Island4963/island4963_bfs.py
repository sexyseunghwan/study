import sys
from queue import Queue

input = sys.stdin.readline

dr = [0,0,1,-1,1,-1,1,-1]
dc = [1,-1,0,0,1,-1,-1,1]

def bfs(r,c):

    visited_map[r][c] = 1 

    queue = Queue()
    queue.put((r,c))

    while not queue.empty():

        x,y = queue.get()

        for i in range (0,8):
            nr = x + dr[i]
            nc = y + dc[i]

            if (nr >= 0 and nc >= 0 and nr < N and nc < M and island_map[nr][nc] != 0 and visited_map[nr][nc] == 0):
                queue.put((nr,nc))
                visited_map[nr][nc] = 1  

while (True):
    
    island_cnt = 0
    M, N = map(int,input().split())

    if (N == M == 0):
        break   
    
    visited_map = [[0] * M for _ in range(N)]
    island_map = [list(map(int, input().split())) for _ in range(N)]

    for i in range (0,N*M):
        
        ind_r = i//M
        ind_c = i%M
        
        if (island_map[ind_r][ind_c] != 0 and visited_map[ind_r][ind_c] == 0):
            bfs(ind_r, ind_c)
            island_cnt += 1

    
    print(island_cnt)


