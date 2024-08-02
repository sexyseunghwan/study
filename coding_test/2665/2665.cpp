#include <iostream>
#include <queue>
#include <algorithm>

using namespace std;

int N;
int map[50][50];
int count_map[50][50];
//int count_map[50][50];
int dr[4] = {0,0,-1,1};
int dc[4] = {1,-1,0,0};


void bfs(int r, int c)
{
    //count_map[r][c] = 0;
    queue<pair<int, int>> que;
    que.push(make_pair(r,c));

    while(!que.empty())
    {
        int row = que.front().first;
        int col = que.front().second;
        que.pop();
        int chg_cnt = map[row][col];

        for (int i = 0; i < 4; i++)
        {
            int new_r = row + dr[i];
            int new_c = col + dc[i];
            
            if (new_r >= 0 && new_r < N && new_c >= 0 && new_c < N && count_map[new_r][new_c] == 0)
            {
                            
            }
        }
    }
}   


int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    cin >> N;
    
    for (int i = 0; i < N*N; i++) cin >> map[i/N][i%N];
    
    bfs(0,0);    
    
    return 0;
}