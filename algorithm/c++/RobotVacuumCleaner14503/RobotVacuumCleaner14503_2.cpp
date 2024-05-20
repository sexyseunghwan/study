#include <iostream>
#include <stack>

using namespace std;

int N,M,c,r,v;
int cleanCnt;
int map[51][51];
int dr[4] = {-1,0,1,0};
int dc[4] = {0,1,0,-1};

int backTurn(int v)
{
    int backV = (v + 2) % 4;
    return backV;
}

int leftTurn(int v)
{
    int leftV = (v + 3) % 4;
    return leftV;
}

void dfs(int r, int c, int v, int plus)
{   
    if (plus == 1)
    {
        cleanCnt++;
        map[r][c] = 2;
    }
    
    bool flag = false;

    for (int i = 0; i < 4; i++)
    {
        int newR = r + dr[i];
        int newC = c + dc[i];

        if (map[newR][newC] == 0)
        {
            flag = true;
            break;
        }
    }
    
    if (flag)
    {
        int leftV = leftTurn(v);

        int newR = r + dr[leftV];
        int newC = c + dc[leftV];   

        if (map[newR][newC] == 0) dfs(newR, newC, leftV, 1);
        else dfs(r, c, leftV, 0);
    }
    else
    {
        int backV = backTurn(v);
        int backR = r + dr[backV];
        int backC = c + dc[backV];

        if (map[backR][backC] != 1) dfs(backR, backC, v, 0);
    }

    return;
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