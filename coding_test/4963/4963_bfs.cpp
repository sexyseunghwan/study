#include <iostream>
#include <cstring>
#include <queue>

using namespace std;

int N,M;
int map[50][50];
int visited[50][50];
int dr[8] = {0,0,1,-1,-1,-1,1,1};
int dc[8] = {1,-1,0,0,-1,1,1,-1};


void bfs(int r, int c)
{
    queue<pair<int, int>> que;
    que.push(make_pair(r,c));
    visited[r][c] = 1;
    
    while(!que.empty())
    {
        int row = que.front().first;
        int col = que.front().second;
        que.pop();

        for (int i = 0; i < 8; i++)
        {
            int nr = row + dr[i];
            int nc = col + dc[i];
            
            if (nr >= 0 && nr < N && nc >= 0 && nc < M && visited[nr][nc] == 0 && map[nr][nc] == 1) 
            {
                que.push(make_pair(nr, nc)); 
                visited[nr][nc] = 1;
            }
        }
    }
}

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);

    while(true)
    {
        cin >> M >> N;

        if (N == 0 && M == 0) break;
        
        for (int i = 0; i < M*N; i++)
        {
            int row = i / M;
            int col = i % M;

            cin >> map[row][col];
        }

        int safety_cnt = 0;

        for (int i = 0; i < M*N; i++)
        {
            int row = i / M;
            int col = i % M;

            if (map[row][col] == 1 && visited[row][col] == 0) {
                safety_cnt++;
                bfs(row, col);
            }
        }

        cout << safety_cnt << endl;
        memset(visited, 0, sizeof(visited));
    }
    
    return 0;
}