#include <iostream>
#include <algorithm>
#include <vector>
#include <queue>

using namespace std;

int N,M;
int max_safety_area;
int map[8][8];
vector<pair<int,int>> virus_list;
int dr[4] = {0,1,0,-1};
int dc[4] = {1,0,-1,0};

void bfs()
{
    // 동적할당
    // int **virus_map = new int*[N];
    // for (int i = 0; i < N; i++) virus_map[i] = new int[M];
    
    int virus_map[8][8];
    for (int i = 0; i < N*M; i++) virus_map[i/M][i%M] = map[i/M][i%M];

    queue<pair<int,int>> que;
    for (const pair<int,int> &pairs : virus_list) que.push({pairs.first, pairs.second});
    
    while(!que.empty())
    {
        auto [r,c] = que.front();
        que.pop();

        for (int i = 0; i < 4; i++)
        {
            int nr = r + dr[i];
            int nc = c + dc[i];

            if (nr >= 0 && nc >= 0 && nr < N && nc < M && virus_map[nr][nc] == 0)
            {
                virus_map[nr][nc] = 2;
                que.push({nr,nc});
            }
        }
    }
    
    int safety_area = 0;
    
    for (int i = 0; i < N*M; i++)
        if (virus_map[i/M][i%M] == 0) safety_area++;
    
    max_safety_area = max(safety_area, max_safety_area);

    // for (int i = 0; i < N; ++i) {
    //     delete[] virus_map[i];
    // }

    // delete[] virus_map;
    
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
        int r = i/M;
        int c = i%M;
        
        if (map[r][c] == 0)
        {
            map[r][c] = 1;
            dfs(i + 1, cnt + 1);
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
        int input_num;
        cin >> input_num;
        
        if (input_num == 2) virus_list.push_back({i/M,i%M});
        map[i/M][i%M] = input_num;
    }
    
    dfs(0,0);
    
    cout << max_safety_area << endl;
    
    return 0;
}