#include <iostream>
#include <algorithm>

using namespace std;

int N;
int map[500][500];
int dp[500][500];
int dr[4] = {1,0,-1,0};
int dc[4] = {0,1,0,-1};
int max_moving;

int dynamicProg(int r, int c)
{
    if (dp[r][c] != 0) return dp[r][c];
    dp[r][c] = 1;

    for (int i = 0; i < 4; i++)
    {
        int n_r = r + dr[i];
        int n_c = c + dc[i];

        if (n_r >= 0 && n_c >= 0 && n_r < N && n_c < N && map[n_r][n_c] > map[r][c])
        {
            dp[r][c] = max(dp[r][c], dynamicProg(n_r, n_c) + 1);
        }
    }

    return dp[r][c];
}

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);  
    
    cin >> N;

    for (int i = 0; i < N*N; i++) cin >> map[i/N][i%N];
    
    for (int i = 0; i < N*N; i++)
    {
        int row = i / N;
        int col = i % N;
        
        if (dp[row][col] == 0) max_moving = max(max_moving, dynamicProg(row, col));
    }
    
    cout << max_moving << endl;

    return 0;
}