#include <iostream>
#include <queue>
#include <utility>
#include <string>

using namespace std;


int N,M;
int maze[100][100];
bool visited[100][100];
int dr[4] = {1,0,-1,0};
int dc[4] = {0,1,0,-1};

void bfs(int r, int c)
{
    visited[r][c] = true;
    queue<pair<int, int>> maze_q;
    maze_q.push(make_pair(r,c));
    
    while (!maze_q.empty())
    {
        int cur_r = maze_q.front().first;
        int cur_c = maze_q.front().second;
        int cur_cnt = maze[cur_r][cur_c];

        maze_q.pop();

        if (cur_r == N-1 && cur_c == M-1) 
        {
            cout << maze[N-1][M-1] << endl;
            break;
        }

        for (int i = 0; i < 4; i++)
        {
            int new_r = cur_r + dr[i];
            int new_c = cur_c + dc[i];

            if (new_r >= 0 && new_c >= 0 && new_r < N && new_c < M && maze[new_r][new_c] != 0 && !visited[new_r][new_c]) 
            {
                maze_q.push(make_pair(new_r,new_c));
                visited[new_r][new_c] = true;
                maze[new_r][new_c] = cur_cnt + 1;
            } 
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
    
    bfs(0,0);

    
    return 0;
}
