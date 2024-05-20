#include <iostream>
#include <algorithm>

using namespace std;

int N,M,c,r,v;
int cleanCnt;
int map[50][50];
int dr[4] = {-1,0,1,0};
int dc[4] = {0,1,0,-1};


int backTurn(int v)
{
    
}

int leftTurn(int v)
{
    
}

void dfs(int r, int c, int v, int plus)
{   
    
    if (plus == 1)
    {
        map[r][c] = 2;
        cleanCnt++;
    } 
    
    bool isTrash = false;
    
    for (int i = 0; i < 4; i++)
    {
        int newR = r + dr[i];
        int newC = c + dc[i];

        if (map[newR][newC] == 0)
        {
            isTrash = true;
            break;
        }
    }
    
    // 쓰레기가 있는 경우  
    if (isTrash)
    {  
        // 시계반대 방향으로 90도 회전
        int nextV = (v-1) < 0 ? 3 : v-1;
        
        int frontR = r + dr[nextV];
        int frontC = c + dc[nextV];

        if (map[frontR][frontC] == 0) dfs(frontR, frontC, nextV, 1);
        else dfs(r, c , nextV, 0);
 
    } 
    // 쓰레기가 없는 경우
    else 
    {   
        int backV = (v+2 >= 4) ? (v+2) % 4 : v+2;
        
        int backR = r + dr[backV];
        int backC = c + dc[backV];

        if (map[backR][backC] != 1) dfs(backR, backC, v, 0);
    }
}


int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    cin >> N >> M >> r >> c >> v;

    for (int i = 0; i < N*M; i++) cin >> map[i/M][i%M];
    
    dfs(r,c,v,1);
    
    cout << cleanCnt << endl;

    return 0;
}