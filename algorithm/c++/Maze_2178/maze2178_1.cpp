#include <iostream>
#include <queue>
#include <utility>
#include <string>

using namespace std;

int N,M;
int maze[100][100];
bool visited[100][100];
int dr[4] = {0,1,0,-1};
int dc[4] = {1,0,-1,0};

void bfs(int cur_r, int cur_c)
{
    queue<pair<int,int>> maze_q;
    visited[cur_r][cur_c] = true;
    maze_q.push(make_pair(cur_r, cur_c));

    while(!maze_q.empty())
    {
        int cur_r = maze_q.front().first;
        int cur_c = maze_q.front().second;
        maze_q.pop();

        if (cur_r == N-1 && cur_c == M-1) break;

        for (int i = 0; i < 4; i++)
        {
            int new_r = cur_r + dr[i];
            int new_c = cur_c + dc[i];

            if (new_r >= 0 && new_c >= 0 && new_r < N && new_c < M && maze[new_r][new_c] != 0 && !visited[new_r][new_c])
            {
                visited[new_r][new_c] = true;
                maze_q.push(make_pair(new_r, new_c));
                maze[new_r][new_c] = maze[cur_r][cur_c] + 1;
            }
        }
    }
}

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);

    cin >> N >> M;

    for (int i = 0; i < N; i++)
    {
        string inputs;
        cin >> inputs;

        for (int j = 0; j < inputs.length(); j++)
        {
            maze[i][j] = inputs[j] - '0';
        }

    }

    bfs(0,0);

    cout << maze[N-1][M-1] << endl;

    return 0;
}