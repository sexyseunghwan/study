#include <iostream>
#include <queue>
#include <algorithm>

using namespace std;

int N,M;
int map[8][8];
int copy_map[8][8];
vector<pair<int,int>> virus_list;
int dr[4] = {0,0,-1,1};
int dc[4] = {1,-1,0,0};
int max_cnt;

void virus_spread()
{
    copy(&map[0][0], &map[0][0] + 8*8, &copy_map[0][0]);
    
    queue<pair<int,int>> que;
    
    for (pair elem : virus_list)
    {
        que.push(make_pair(elem.first, elem.second));
        
        while(!que.empty())
        {
            int r = que.front().first;
            int c = que.front().second;
            que.pop();

            for (int i = 0; i < 4; i++)
            {
                int new_r = r + dr[i];
                int new_c = c + dc[i];

                if (new_r >= 0 && new_r < N && new_c >= 0 && new_c < M && copy_map[new_r][new_c] == 0)
                {
                    copy_map[new_r][new_c] = 2;
                    que.push(make_pair(new_r, new_c));
                }
            }
        }
    }
    
    int cnt = 0;

    for (int i = 0; i < N*M; i++)
    {
        int r = i / M;
        int c = i % M;

        if (copy_map[r][c] == 0) cnt++;
    }
    
    max_cnt = cnt > max_cnt ? cnt : max_cnt;
}


void make_wall(int idx, int wall_cnt)
{
    if (wall_cnt == 3)
    {
        virus_spread();
        return;
    }
    
    for (int i = idx; i < N*M; i++)
    {
        int r = i / M;
        int c = i % M;

        if (map[r][c] == 0)
        {
            map[r][c] = 1;
            make_wall(i + 1, wall_cnt + 1);
            map[r][c] = 0;
        }
    }
        
}


int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    cin >> N >> M;

    for (int i = 0; i < N*M; i++)
    {
        int r = i / M;
        int c = i % M;

        int input;
        cin >> input;

        if (input == 2) virus_list.push_back(make_pair(r,c));

        map[r][c] = input;
    }
    
    make_wall(0,0); 
    
    cout << max_cnt << endl;

    return 0;
}