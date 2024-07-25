#include <iostream>
#include <queue>
#include <algorithm>

using namespace std;

int N,M;
int map[8][8];
int copy_map[8][8];
vector<pair<int,int>> virus_list;
int dr[4] = {1,-1,0,0};
int dc[4] = {0,0,1,-1};
int max_cnt;


void dfs_v(int r, int c) 
{

    copy_map[r][c] = 2;

    for (int i = 0; i < 4; i++)
    {
        int new_dr = r + dr[i];
        int new_dc = c + dc[i];

        if (new_dr >= 0 && new_dr < N && new_dc >= 0 && new_dc < M && copy_map[new_dr][new_dc] == 0) 
        {
            dfs_v(new_dr, new_dc);
        } 
    }
}


void virus_check() 
{
    
    copy(&map[0][0], &map[0][0] + 8*8, &copy_map[0][0]);
    
    for (pair elem : virus_list)
    {
        dfs_v(elem.first, elem.second);    
    }
    
    int safe_cnt = 0;
    
    for (int i = 0; i < N; i++)
    {
        for (int j = 0; j < M; j++) {
            if (copy_map[i][j] == 0) { safe_cnt++; }          
        }
    }

    // cout << "=======copy========" << endl;
    // for (int i = 0; i < N; i++) {
    //     for (int j = 0; j < M; j++) {
    //         cout << copy_map[i][j] << " ";
    //     }
    //     cout << endl;
    // }
    // cout << "========copy=======" << endl;


    max_cnt = safe_cnt > max_cnt ? safe_cnt : max_cnt;
}


void dfs_1(int idx, int safe_cnt) 
{
    if (safe_cnt == 3) 
    {
        // for (int i = 0; i < N; i++) {
        //     for (int j = 0; j < M; j++) {
        //         cout << map[i][j] << " ";
        //     }
        //     cout << endl;
        // }
        // cout << "===============" << endl;
        
        virus_check();
        return;
    }
    
    for (int i = idx; i < N*M; i++) 
    {
        int r = i / M;
        int c = i % M;

        if (map[r][c] == 0) 
        {
            map[r][c] = 1;
            dfs_1(i + 1, safe_cnt + 1);
            map[r][c] = 0;
        }
    }
}

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    cin >> N >> M;

    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            int input;
            cin >> input;
            map[i][j] = input;
            if (input == 2) { virus_list.push_back(make_pair(i,j)); }
        }
    }
    
    dfs_1(0,0);

    cout << max_cnt << endl;

    return 0;
}