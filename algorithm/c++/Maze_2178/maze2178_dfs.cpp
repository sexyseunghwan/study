#include <iostream>
#include <string>

using namespace std;

int N,M;
int maze[100][100];
bool visited[100][100];
int dr[4] = {1,0,-1,0};
int dc[4] = {0,1,0,-1};

int dfs(int cur_r, int cur_c, int count)
{
    visited[cur_r][cur_c] = true;
    
    if (cur_r == N-1 && cur_c == M-1) return count;

    for (int i = 0; i < 4; i++)
    {
        int new_r = cur_r + dr[i];
        int new_c = cur_c + dc[i];

        if (new_r >= 0 && new_c >= 0 && new_r < N && new_c < M && maze[new_r][new_c] == 1 && !visited[new_r][new_c]) 
        {
            return dfs(new_r, new_c, count+1);
        }
    }
}


int main()
{
    cin >> N >> M;
    
    for (int i = 0; i < N; i++)
    {
        string inputs;
        int index = 0;
        cin >> inputs;

        for (char input : inputs) 
        {
            maze[i][index++] = input - '0';
        }
    }
    
    cout << dfs(0,0,1);
    
    return 0;
}