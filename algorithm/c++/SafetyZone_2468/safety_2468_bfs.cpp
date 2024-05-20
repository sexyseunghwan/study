#include <iostream>
#include <cstring>
#include <queue>
#include <utility>
#include <algorithm>

using namespace std;

int N;
int map[100][100];
bool visited[100][100];
int max_cnt = 1;
int max_n;
int dr[4] = {0,1,0,-1};
int dc[4] = {1,0,-1,0};

void bfs(int r, int c, int h)
{
    queue<pair<int, int>> map_q;
    map_q.push(make_pair(r,c));
    visited[r][c] = true;
    
    while (!map_q.empty())
    {
        auto [cur_r, cur_c] = map_q.front();
        map_q.pop();
        
        for (int i = 0; i < 4; i++)
        {
            int new_r = cur_r + dr[i];
            int new_c = cur_c + dc[i];

            if (new_r >= 0 && new_c >= 0 && new_r < N && new_c < N && map[new_r][new_c] > h && !visited[new_r][new_c])
            {
                map_q.push(make_pair(new_r, new_c));
                visited[new_r][new_c] = true;
            }
        }
    }
}


int main()
{
    cin >> N;

    for (int i = 0; i < N*N; i++)
    {
        int input_n; cin >> input_n;
        max_n = max(input_n, max_n);
        map[i/N][i%N] = input_n;
    }
    
    for (int i = 1; i <= max_n; i++)
    {
        int cnt = 0;

        for (int j = 0; j < N*N; j++)
        {
            int cur_r = j/N;
            int cur_c = j%N;

            if (map[cur_r][cur_c] > i && !visited[cur_r][cur_c])
            {
                bfs(cur_r, cur_c, i);
                cnt++;
            }    
        }

        max_cnt = max(cnt, max_cnt);
        memset(visited, false, sizeof(visited));
    }
    
    cout << max_cnt << endl;

    return 0;
}