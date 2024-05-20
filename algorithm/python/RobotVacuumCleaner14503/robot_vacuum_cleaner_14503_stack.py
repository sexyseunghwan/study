import sys
input = sys.stdin.readline

class Elem:
    def __init__(self, r, c, v, p):
        self.r = r
        self.c = c
        self.v = v
        self.p = p

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

    stack = []
    stack.append(Elem(r, c, v, p))

    while stack:

        elem = stack.pop()

        if (elem.p == 1):
            global clear_cnt
            clear_cnt += 1
            r_map[elem.r][elem.c] = 2
        
        flag = False

        for i in range(0,4):
            next_r = elem.r + dr[i]
            next_c = elem.c + dc[i]

            if (r_map[next_r][next_c] == 0):
                flag = True
                break
        
        if (flag):
            front_v = turn_left(elem.v)
            
            front_r = elem.r + dr[front_v]
            front_c = elem.c + dc[front_v]

            if (r_map[front_r][front_c] == 0):
                stack.append(Elem(front_r, front_c, front_v, 1))
            else:
                stack.append(Elem(elem.r, elem.c, front_v, 0))
            
        else:
            back_v = turn_back(elem.v)
            
            back_r = elem.r + dr[back_v]
            back_c = elem.c + dc[back_v]

            if (r_map[back_r][back_c] != 1):
                stack.append(Elem(back_r, back_c, elem.v, 0))
    

dfs(r,c,v,1)
print(clear_cnt)

