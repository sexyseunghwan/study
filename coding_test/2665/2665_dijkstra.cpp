#include <iostream>
#include <queue>
#include <algorithm>
#include <climits>

using namespace std;

int N;
int map[50][50];
int count_map[50][50];
int dr[4] = {0,0,-1,1};
int dc[4] = {1,-1,0,0};

void dijkstra(int r, int c)
{
    priority_queue<pair< int, pair< int, int > > > p_que;
    p_que.push(make_pair(0,make_pair(0,0)));
    count_map[r][c] = 0;
    
    while(!p_que.empty())
    {
        int bias = -p_que.top().first;
        r = p_que.top().second.first;
        c = p_que.top().second.second;
        p_que.pop();
        
        for (int i = 0; i < 4; i++)
        {
            int nr = r + dr[i];
            int nc = c + dc[i];

            if (nr >= 0 && nc >= 0 && nr < N && nc < N)
            {
                if (count_map[nr][nc] > map[r][c] + bias)
                {
                    count_map[nr][nc] = map[r][c] + bias;
                    p_que.push(make_pair(-count_map[nr][nc], make_pair(nr, nc)));
                } 
            }
        }
    }
}

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);

    cin >> N;

    for (int i = 0; i < N; i++ ) {
        char ch[N+1];
        cin >> ch;
        
        for (int j = 0; j < N; j++) {
            int input = ch[j] - '0';
            if (input == 1) map[i][j] = 0;
            else  map[i][j] = 1;
        }
    }

    for (int i = 0; i < N*N; i++) 
    {
        count_map[i/N][i%N] = INT_MAX;
    }
    
    dijkstra(0,0);

    cout << count_map[N-1][N-1] << endl;
}