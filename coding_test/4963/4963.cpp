#include <iostream>

using namespace std;

int N,M;
int map[50][50];
int visitied_map[50][50];
int dr[8] = {0,0,1,-1,-1,-1,1,1};
int dc[8] = {1,-1,0,0,-1,1,1,-1};

void init_map()
{
    for (int i = 0; i < N*M; i++) visitied_map[i/M][i%M] = 0;
}


void dfs(int r, int c)
{

    if (map[r][c] == 0 || visitied_map[r][c] == 1) return;

    visitied_map[r][c] = 1;

    for (int i = 0; i < 8; i++)
    {
        int nr = r + dr[i];
        int nc = c + dc[i];

        if (nr >= 0 && nr < N && nc >= 0 && nc < M && visitied_map[nr][nc] == 0 && map[nr][nc] == 1)
        {
            dfs(nr, nc);
        }
    }
    
}


int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);    
    
    
    while(true)
    {
        cin >> M >> N;

        if (N == 0 && M == 0) break; 

        init_map();
        int island_cnt = 0;
        for (int i = 0; i < N*M; i++) cin >> map[i/M][i%M];

        for (int i = 0; i < N*M; i++)
        {
            int row = i/M;
            int col = i%M;

            if (map[row][col] == 1 && visitied_map[row][col] == 0) 
            {
                island_cnt++;
                dfs(row, col);
            }
        }

        cout << island_cnt << endl;
        
    }
    
    return 0;
}