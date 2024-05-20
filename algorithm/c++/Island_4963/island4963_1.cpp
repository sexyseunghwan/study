#include <iostream>
#include <stack>
#include <utility>

using namespace std;

int N,M;
int** map;
int dr[8] = {0,0,1,-1,1,-1,1,-1}; //행에 관한 방향데이터
int dc[8] = {1,-1,0,0,1,-1,-1,1}; //열에 관한 방향데이터


void dfs(int r, int c)
{
    stack<pair<int, int>> map_stack;
    map_stack.push(make_pair(r,c));
    map[r][c] = 0;

    while (!map_stack.empty())
    {
        int r = map_stack.top().first;
        int c = map_stack.top().second;
        map_stack.pop();

        for (int i = 0; i < 8; i++) 
        {
            int nr = r + dr[i];
            int nc = c + dc[i];

            if (nr >= 0 && nc >= 0 && nr < N && nc < M && map[nr][nc] != 0) 
            {
                map_stack.push(make_pair(nr,nc));
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
        
        
        map = new int*[N];

        for (int i = 0; i < N; i++) map[i] = new int[M];
        for (int i = 0; i < N*M; i++) cin >> map[i/M][i%M]; 
        
        for (int i = 0; i < N*M; i++)
        {   
            int r = i/M;
            int c = i%M;

            if (map[r][c] != 0)
            {
                dfs(r,c);
                island_cnt++;
            }
        }

        cout << island_cnt << endl;
        
    }

    return 0;
}