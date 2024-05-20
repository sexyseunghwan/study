#include <iostream>
#include <algorithm>
#include <utility>
#include <stack>
#include <cstring>

using namespace std;

int N;
int map[100][100];
bool visited[100][100];
int dr[4] = {0,1,0,-1};
int dc[4] = {1,0,-1,0};
int max_cnt;
int min_h;
int max_h;

void dfs(int r, int c, int h)
{
    stack<pair<int,int>> stack;
    stack.push(make_pair(r,c));
    visited[r][c] = true;
    
    while(!stack.empty())
    {
        auto [cur_r, cur_c] = stack.top();
        stack.pop();
        
        for (int i = 0; i < 4; i++)
        {
            int new_r = cur_r + dr[i];
            int new_c = cur_c + dc[i];

            if (new_r >= 0 && new_c >= 0 && new_r < N && new_c < N && map[new_r][new_c] > h && !visited[new_r][new_c])
            {
                stack.push(make_pair(new_r, new_c));
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
        int h; cin >> h;
        max_h = max(h,max_h);
        min_h = min(h,min_h);

        map[i/N][i%N] = h;
    }

    for (int h = min_h-1; h < max_h; h++)
    {
        int cnt = 0;

        for (int i = 0; i < N*N; i++)
        {
            int cur_r = i / N;
            int cur_c = i % N;

            if (map[cur_r][cur_c] > h && !visited[cur_r][cur_c])
            {
                dfs(cur_r, cur_c, h);
                cnt++;
            }    
        }
        
        max_cnt = max(cnt, max_cnt);
        memset(visited, false, sizeof(visited));
    }
    
    cout << max_cnt << endl;

    return 0;
}