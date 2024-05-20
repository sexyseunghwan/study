#include <iostream>
#include <queue>

using namespace std;

int N,M,c,r,v;
int cleanCnt;
int map[51][51];
int dr[4] = {-1,0,1,0};
int dc[4] = {0,1,0,-1};

struct Elem {
    int r,c,v,p;
};

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

void bfs(int r, int c, int v, int p)
{   
    
    queue<Elem> queue;
    queue.push({r, c, v, p});

    while(!queue.empty())
    {
        Elem elem = queue.front();
        queue.pop();

        if (elem.p == 1)
        {
            cleanCnt++;
            map[elem.r][elem.c] = 2; 
        }

        bool flag = false;

        for (int i = 0; i < 4; i++)
        {
            int newR = elem.r + dr[i];
            int newC = elem.c + dc[i];

            if (map[newR][newC] == 0)
            {
                flag = true;
                break;
            }
        }

        if (flag)
        {
            int leftV = leftTurn(elem.v);

            int newR = elem.r + dr[leftV];
            int newC = elem.c + dc[leftV];   

            if (map[newR][newC] == 0) queue.push({newR, newC, leftV, 1});
            else queue.push({elem.r, elem.c, leftV, 0});
        }
        else
        {
            int backV = backTurn(elem.v);
            int backR = elem.r + dr[backV];
            int backC = elem.c + dc[backV];

            if (map[backR][backC] != 1) queue.push({backR, backC, elem.v, 0});
        }
    }
}


int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    cin >> N >> M >> r >> c >> v;

    for (int i = 0; i < N*M; i++) cin >> map[i/M][i%M];
    
    bfs(r,c,v,1);
    
    cout << cleanCnt << endl;

    return 0;
}