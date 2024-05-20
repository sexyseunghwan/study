#include <iostream>
#include <queue>
#include <utility>
#include <cstring> // use memset

using namespace std;

int N,M;
int map[50][50];
bool visited_map[50][50];
int dr[8] = {0,0,1,-1,1,-1,1,-1}; //행에 관한 방향데이터
int dc[8] = {1,-1,0,0,1,-1,-1,1}; //열에 관한 방향데이터


void bfs(int r, int c)
{
    queue<pair<int, int>> map_q;
    map_q.push(make_pair(r,c));
    visited_map[r][c] = true;

    while (!map_q.empty())
    {
        int cur_r = map_q.front().first;
        int cur_c = map_q.front().second;   
        map_q.pop();
        
        for (int i = 0; i < 8; i++) 
        {
            int new_r = cur_r + dr[i];
            int new_c = cur_c + dc[i];

            if (new_r >= 0 && new_c >= 0 && new_r < N && new_c < M && map[new_r][new_c] != 0 && !visited_map[new_r][new_c]) 
            {
                map_q.push(make_pair(new_r,new_c));
                visited_map[new_r][new_c] = true;
            } 
        }

    }
}


int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);

    while(1)
    {
        cin >> M >> N;
        
        int island_cnt = 0;

        if (M == 0 && N == 0) break;

        for (int i = 0; i < N*M; i++) cin >> map[i/M][i%M]; 
        
        for (int i = 0; i < N*M; i++)
        {   
            int r = i/M;
            int c = i%M;

            if (map[r][c] != 0 && !visited_map[r][c])
            {
                bfs(r,c);
                island_cnt++;
            }
        }

        cout << island_cnt << endl;
        memset(map, false, sizeof(map));
        memset(visited_map, false, sizeof(visited_map));
    }

    

    return 0;
}