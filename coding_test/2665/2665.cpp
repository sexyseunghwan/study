#include <iostream>
#include <queue>
#include <algorithm>

using namespace std;

int N;
int map[50][50];
int count_map[50][50];
int visited_map[50][50];
int dr[4] = {0,0,-1,1};
int dc[4] = {1,-1,0,0};


void bfs(int r, int c)
{
    queue<pair<int, int>> que;
    que.push(make_pair(r,c));
    visited_map[r][c] = 1;

    while(!que.empty())
    {
        int row = que.front().first;
        int col = que.front().second;
        que.pop();

        for (int i = 0; i < 4; i++)
        {
            int new_r = row + dr[i];
            int new_c = col + dc[i];
            
            if (new_r >= 0 && new_r < N && new_c >= 0 && new_c < N)
            {
                int count_cnt = count_map[r][c];
                if (map[new_r][new_c] == 0) count_cnt++;
                
                if (visited_map[new_r][new_c] == 0) {
                    count_map[new_r][new_c] = count_cnt;
                    visited_map[new_r][new_c] = 1;
                    que.push(make_pair(new_r, new_c));
                } else if (count_map[new_r][new_c] > count_cnt) {
                    count_map[new_r][new_c] = count_cnt;
                    que.push(make_pair(new_r, new_c));
                }   
            }
        }
    }
}   


int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    cin >> N;
    
    for (int i = 0; i < N*N; i++) cin >> map[i/N][i%N];
    
    bfs(0,0);    
    
    cout << count_map[N-1][N-1] << endl;

    return 0;
}