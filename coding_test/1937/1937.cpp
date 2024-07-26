#include <iostream>
#include <cstring>
#include <algorithm>

using namespace std;

int N;
int map[500][500];
int dp[500][500];
int max_cnt;
int dr[4] = {0,0,1,-1};
int dc[4] = {1,-1,0,0};

int dfs(int r, int c)
{
    if (dp[r][c] != 0)
    {
        return dp[r][c];
    } 
    
    dp[r][c] = 1;

    for (int i = 0; i < 4; i++)
    {
        int nr = r + dr[i];
        int nc = c + dc[i];

        if (nr >= 0 && nr < N && nc >= 0 && nc < N && map[nr][nc] > map[r][c])
        {
            dp[r][c] = max(dp[r][c], dfs(nr, nc) + 1);
        }
    }

    return dp[r][c];
}


int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);

    cin >> N;
    
    for (int i = 0; i < N*N; i++)
    {
        int r = i / N;
        int c = i % N;

        cin >> map[r][c];
    }    

    for (int i = 0; i < N*N; i++)
    {
        int r = i / N;
        int c = i % N;
        
        dp[r][c] = max(dp[r][c], dfs(r,c));
    }
    
    for (int i = 0; i < N*N; i++)
    {
        int cnt = dp[i/N][i%N];
        max_cnt = cnt > max_cnt ? cnt : max_cnt;
    }

    cout << max_cnt << endl;

    return 0;
}