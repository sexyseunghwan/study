#include <iostream>
#include <queue>
#include <algorithm>
#include <climits>

using namespace std;

int N;
int map[50][50];
int count_map[50][50];
int dr[4] = {0,0,-1,1};
int dc[4] = {1,-1,0,0};


void bfs(int r, int c)
{
    queue<pair<int, int>> que;
    que.push(make_pair(r,c));
    count_map[r][c] = 0;

    while(!que.empty())
    {
        int r = que.front().first;
        int c = que.front().second;
        que.pop();

        for (int i = 0; i < 4; i++)
        {
            int new_r = r + dr[i];
            int new_c = c + dc[i];
            
            if (new_r >= 0 && new_r < N && new_c >= 0 && new_c < N)
            {
                if (map[new_r][new_c] == 1) {
                    
                    if (count_map[new_r][new_c] > count_map[r][c])
                    {
                        count_map[new_r][new_c] = count_map[r][c];
                        que.push(make_pair(new_r, new_c));
                    }
                    
                } else {
                    
                    if (count_map[new_r][new_c] > count_map[r][c] + 1)
                    {
                        count_map[new_r][new_c] = count_map[r][c] + 1;
                        que.push(make_pair(new_r, new_c));
                    }
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
    
    for (int i = 0; i < N; i++ ) {
        char ch[N+1];
        cin >> ch;
        
        for (int j = 0; j < N; j++) {
            int input = ch[j] - '0';
            if (input == 1) map[i][j] = 1;
            else  map[i][j] = 0;
        }
    }
    
    for (int i = 0; i < N*N; i++) 
    {
        count_map[i/N][i%N] = INT_MAX;
    }
    
    bfs(0,0);

    cout << count_map[N-1][N-1] << endl;

    return 0;
}