#include <iostream>
#include <queue>
#include <algorithm>
#include <cstring>

using namespace std;

int N, M, max_dist;
char map[50][50];
int visited[50][50];
int dx[4] = {0,0,1,-1};
int dy[4] = {1,-1,0,0};

int bfs(int x, int y)
{   
    queue<pair<int,int>> que;
    que.push(make_pair(x, y));
    visited[x][y] = 1;
    int inner_max_dist = 0;

    while(!que.empty())
    {
        int inner_x = que.front().first;
        int inner_y = que.front().second;
        que.pop();
        
        for (int i = 0; i < 4; i++)
        {
            int new_x = inner_x + dx[i];
            int new_y = inner_y + dy[i];
            
            if (new_x >= 0 && new_y >= 0 && new_x < N && new_y < M && visited[new_x][new_y] == 0 && map[new_x][new_y] == 'L')
            {
                visited[new_x][new_y] = visited[inner_x][inner_y] + 1;
                inner_max_dist = max(visited[new_x][new_y], inner_max_dist);

                que.push(make_pair(new_x, new_y));
            }
        }
    }


    return inner_max_dist;
}

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    cin >> N >> M;

    for (int i = 0; i < N*M; i++)
    {
        char input;
        cin >> input;

        map[i/M][i%M] = input;
    }
    
    for (int i = 0; i < N; i++)
    {
        for (int j = 0; j < M; j++)
        {
            if (map[i][j] == 'L')
            {
                int cur_dist = bfs(i, j);
                max_dist = max(cur_dist, max_dist);
                memset(visited, 0, sizeof(visited));
            }
        }
    }

    cout << max_dist - 1 << endl;

    return 0;
}