import sys
input = sys.stdin.readline

clear_cnt = 0
dr = [-1,0,1,0]
dc = [0,1,0,-1]

N, M = map(int,input().split())
r_map = [[0 for _ in range(M)] for _ in range(N)]
r,c,v = list(map(int, input().split()))

for i in range(0,N):
    input_list = list(map(int, input().split()))
    for j in range(0,M):
        r_map[i][j] = input_list[j]

def turn_left(v):
    left_v = (v + 3) % 4
    return left_v 
    
def turn_back(v):
    back_v = (v + 2) % 4
    return back_v 

def dfs(r, c, v, p):
    
    if (p == 1):
        global clear_cnt
        clear_cnt += 1
        r_map[r][c] = 2
    
    flag = False

    for i in range(0,4):
        next_r = r + dr[i]
        next_c = c + dc[i]

        if (r_map[next_r][next_c] == 0):
            flag = True
            break
    
    if (flag):
        front_v = turn_left(v)
        
        front_r = r + dr[front_v]
        front_c = c + dc[front_v]

        if (r_map[front_r][front_c] == 0):
            dfs(front_r, front_c, front_v, 1)
        else:
            dfs(r, c, front_v, 0)
        
    else:
        back_v = turn_back(v)
        
        back_r = r + dr[back_v]
        back_c = c + dc[back_v]

        if (r_map[back_r][back_c] != 1):
            dfs(back_r, back_c, v, 0)
    
    return


dfs(r,c,v,1)
print(clear_cnt)

