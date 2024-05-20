#include <iostream>
#include <cstring>
#include <algorithm>

using namespace std;

int N;
int map[100][100];
bool visited[100][100];
int dr[4] = {0,1,0,-1};
int dc[4] = {1,0,-1,0};
int max_cnt;
int min_cnt;
int max_h;

void dfs(int r, int c, int h)
{   
    visited[r][c] = true;

    for (int i = 0; i < 4; i++)
    {
        int new_r = r + dr[i];
        int new_c = c + dc[i];

        if (new_r >= 0 && new_c >= 0 && new_r < N && new_c < N && map[new_r][new_c] > h && !visited[new_r][new_c])
        {
            dfs(new_r, new_c, h);
        }
    }
}

int main()
{
    cin >> N;
    
    for (int i = 0; i < N*N; i++)
    {
        int input_h; cin >> input_h;

        max_h = max(max_h, input_h);
        min_cnt = min(min_cnt, input_h);
        map[i/N][i%N] = input_h;
    }
    

    for (int i = min_cnt-1; i < max_h; i++)
    {
        int cnt = 0;

        for (int j = 0; j < N*N; j++)
        {
            int r = j/N;
            int c = j%N;

            if (map[r][c] > i && !visited[r][c])
            {
                dfs(r,c,i);
                cnt++;    
            }
        }

        max_cnt = max(cnt, max_cnt);
        memset(visited, false, sizeof(visited));
    }

    cout << max_cnt << endl;
    
    return 0;
}


