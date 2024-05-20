#include <iostream>
#include <queue>
#include <algorithm>
#include <cstring>

using namespace std;

int N,M;
int max_safety_area;
int map[8][8];
vector<pair<int,int>> virus_list;
int dr[4] = {0,1,0,-1};
int dc[4] = {1,0,-1,0};


void bfs()
{

    int virus_map[8][8];
    for (int i = 0; i < N*M; i++) virus_map[i/M][i%M] = map[i/M][i%M];
    
    queue<pair<int, int>> map_q;
    for (pair<int,int> elem : virus_list) map_q.push(make_pair(elem.first, elem.second));
    
    while(!map_q.empty())
    {
        auto [cur_r, cur_c] = map_q.front();
        map_q.pop();

        for (int i = 0; i < 4; i++)
        {
            int new_r = cur_r + dr[i];
            int new_c = cur_c + dc[i];

            if (new_r >= 0 && new_r < N && new_c >= 0 && new_c < M && virus_map[new_r][new_c] == 0)
            {
                virus_map[new_r][new_c] = 2;
                map_q.push(make_pair(new_r, new_c));
            } 
        }
    }
    
    int safety_area = 0;

    for (int i = 0; i < N*M; i++) 
        if (virus_map[i/M][i%M] == 0) safety_area++;
    
    max_safety_area = max(max_safety_area, safety_area);
}


void dfs(int index, int cnt)
{
    if (cnt == 3) 
    {
        bfs();
        return;
    }
    
    for (int i = index; i < N*M; i++)
    {
        int n_i = i/M;
        int n_j = i%M;

        if (map[n_i][n_j] == 0)
        {
            map[n_i][n_j] = 1;
            dfs(i + 1, cnt + 1);
            map[n_i][n_j] = 0;
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
        int input_r = i/M;
        int input_c = i%M;

        cin >> map[input_r][input_c];

        if (map[input_r][input_c] == 2) virus_list.push_back(make_pair(input_r, input_c));
    }

    dfs(0,0);
    
    cout << max_safety_area << endl;

    return 0;
}