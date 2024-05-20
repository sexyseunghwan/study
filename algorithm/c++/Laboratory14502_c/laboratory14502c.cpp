#include <iostream>
#include <queue>
#include <algorithm>

using namespace std;

int N, M, max_cnt;
int map[8][8];
vector<pair<int,int>> virus_list;
int dr[4] = {0,0,1,-1};
int dc[4] = {1,-1,0,0};


void bfs()
{
    int virus_map[8][8];
    for (int i = 0; i < N*M; i++) virus_map[i/M][i%M] = map[i/M][i%M];

    queue<pair<int,int>> que;
    for (pair elem : virus_list) que.push(make_pair(elem.first, elem.second));

    while(!que.empty())
    {
        int cur_r = que.front().first;
        int cur_c = que.front().second;
        que.pop();

        for (int i = 0; i < 4; i++)
        {
            int new_r = cur_r + dr[i];
            int new_c = cur_c + dc[i];

            if (new_r >= 0 && new_r < N && new_c >= 0 && new_c < M && virus_map[new_r][new_c] == 0) 
            {
                virus_map[new_r][new_c] = 2;
                que.push(make_pair(new_r, new_c));
            }
        }
    }

    int safety_cnt = 0;

    for (int i = 0; i < N*M; i++)
        if (virus_map[i/M][i%M] == 0) safety_cnt++;

    max_cnt = max(max_cnt, safety_cnt);
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
        if (map[i/M][i%M] == 0)
        {
            map[i/M][i%M] = 1;
            dfs(i + 1, cnt + 1);
            map[i/M][i%M] = 0;
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
        int input;
        int in_r = i / M;
        int in_c = i % M;
        cin >> input;

        map[in_r][in_c] = input;

        if (input == 2) virus_list.push_back(make_pair(in_r, in_c));
    }    
         
    dfs(0,0);

    cout << max_cnt << endl;

    return 0;
}