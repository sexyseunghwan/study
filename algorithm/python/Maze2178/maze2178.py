import sys
from queue import Queue

input = sys.stdin.readline

dr = [0,1,0,-1]
dc = [1,0,-1,0]

N, M = map(int, input().split())
maze_map = [list(map(int, input().rstrip())) for _ in range(N)]
visited = [[0] * M for _ in range(N)]

def bfs(cur_r,cur_c):
    visited[cur_r][cur_c] = 1
    
    for i in range (4):
        new_r = cur_r + dr[i]
        new_c = cur_c + dc[i]

        if (new_r >= 0 and new_c >= 0 and new_r < N and new_c < M and visited[new_r][new_c] == 0 and maze_map[new_r][new_c] != 0):
            maze_queue.put([new_r, new_c])
            visited[new_r][new_c] = True
            maze_map[new_r][new_c] = maze_map[cur_r][cur_c] + 1


maze_queue = Queue()
maze_queue.put([0,0])

while(not maze_queue.empty()):
    
    cur_r, cur_c = maze_queue.get()

    if (cur_r == N-1 and cur_c == M-1):     
        print(maze_map[N-1][M-1])        

    bfs(cur_r, cur_c)



