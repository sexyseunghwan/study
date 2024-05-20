#include <iostream>
#include <algorithm>

using namespace std;

int N;
int map[500][500];
int dp[500][500];
int dr[4] = {1,0,-1,0};
int dc[4] = {0,1,0,-1};
int max_moving;

int dynamicProg(int row,int col)
{
    if (dp[row][col] != 0) return dp[row][col];
    dp[row][col] = 1;

    for (int i = 0; i < 4; i++)
    {
        int new_r = row + dr[i];
        int new_c = col + dc[i];

        if (new_r >= 0 && new_c >= 0 && new_r < N && new_c < N && map[new_r][new_c] > map[row][col])
            dp[row][col] = max(dp[row][col], dynamicProg(new_r, new_c) + 1);
        
    }

    return dp[row][col];
}

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);  
    
    cin >> N;

    for (int i = 0; i < N*N; i++) cin >> map[i/N][i%N];

    for (int i = 0; i < N*N; i++) max_moving = max(max_moving, dynamicProg(i/N,i%N));
    
    cout << max_moving << endl;
    
    return 0;
}