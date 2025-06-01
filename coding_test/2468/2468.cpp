#include <iostream>
#include <algorithm>

using namespace std;

int N;
int map[100][100];
int visited[100][100];
int dr[4] = {0,0,-1,1};
int dc[4] = {1,-1,0,0};
int cur_safety_area;
int total_safety_area;
int max_val = 0;
int min_val = 100;

void dfs(int r, int c, int limit) {

    visited[r][c] = 1;

    for (int i = 0; i < 4; i++) {
        int next_r = r + dr[i];
        int next_c = c + dc[i];
        
        if (next_r < N && next_c < N && next_r >= 0 && next_c >= 0 
            && map[next_r][next_c] > limit &&visited[next_r][next_c] == 0) {
            dfs(next_r, next_c, limit);
        }
    }        
}

int main() 
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);

    cin >> N;

    for (int i = 0; i < N*N; i++) {
        int input;
        cin >> input;
        
        min_val = min(min_val, input);
        max_val = max(max_val, input);
    }
    
    for (int i = min_val; i < max_val; i++) {
        for (int j = 0; j < N*N; j++) {
            int r = j / N; 
            int c = j % N;     
        }
    }
    
    

    return 0;
}