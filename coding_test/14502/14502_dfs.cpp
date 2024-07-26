#include <iostream>
#include <queue>
#include <algorithm>

using namespace std;

int N,M;
int map[8][8];
int copy_map[8][8];
vector<pair<int,int>> virus_vec;
int dr[4] = {0,0,1,-1};
int dc[4] = {1,-1,0,0};
int max_cnt;

void virus_dfs(int r, int c)
{   
    copy_map[r][c] = 2;
    
    for (int i = 0; i < 4; i++)
    {
        int new_r = r + dr[i];
        int new_c = c + dc[i];

        if (new_r >= 0 && new_r < N && new_c >= 0 && new_c < M && copy_map[new_r][new_c] == 0) 
        {
            virus_dfs(new_r, new_c);
        } 
    }
}

void virus_spread()
{
    copy(&map[0][0], &map[0][0] + 8*8, &copy_map[0][0]);
    
    for (pair elem : virus_vec)
    {
        int v_r = elem.first;
        int v_c = elem.second;

        virus_dfs(v_r, v_c);
    }
    
    int cnt = 0;

    for (int i = 0; i < N*M; i++)
    {
        int r = i / M;
        int c = i % M;

        if (copy_map[r][c] == 0)
        {
            cnt++;
        }
    }
    
    max_cnt = cnt > max_cnt ? cnt : max_cnt;
    
}

void make_wall(int idx, int cnt)
{
    if (cnt == 3) 
    {
        virus_spread();
        return;
    }

    for (int i = idx; i < N*M; i++)
    {
        int r = i / M;
        int c = i % M;

        if (map[r][c] == 0) {
            map[r][c] = 1;
            make_wall(idx + 1, cnt + 1);
            map[r][c] = 0;
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
        for (int j = 0; j < M; j++)
        {
            int input;
            cin >> input;

            if (input == 2)
            {
                virus_vec.push_back(make_pair(i,j));
            }

            map[i][j] = input;
        }
    }


    make_wall(0,0);

    cout << max_cnt << endl;

    return 0;
}