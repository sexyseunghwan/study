#include <iostream>
#include <queue>
#include <algorithm>
#include <stack>

using namespace std;



int N, M, max_cnt;
int map[8][8];
vector<pair<int,int>> virus_list;
int dr[4] = {0,0,1,-1};
int dc[4] = {1,-1,0,0};

struct State
{
    int index;
    int cnt;
    int row;
    int col;
    //bool processed;
    //int flag_index;

    //State(int i, int c, bool p) : index(i), cnt(c), processed(p){}
    //State(int i, int c) : index(i), cnt(c) {}
    State(int index, int cnt, int row, int col) : index(index), cnt(cnt), row(row), col(col) {}
};

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
    stack<State> s; 
    s.push(State(0,0,0,0));
    
    while (!s.empty()) {
        
        State &cur = s.top(); 
        
        if (cur.cnt == 3) {
            bfs();
            s.pop();
            map[cur.row][cur.col] = 0;

            cout << "======" << endl;
            for (int i = 0; i < N; i++)
            {
                for (int j = 0; j < M; j++)
                {
                    cout << map[i][j] << " ";
                }
                cout << endl;
            }
            cout << "======" << endl;
            
            continue;
        } 

        for (int i = cur.index; i < N*M; i++)
        {
            if (map[i/M][i%M] == 0)
            {
                map[i/M][i%M] = 1;
                s.push(State(i + 1, cur.cnt + 1, i/M, i%M));
                break;
            }
        }
        

        // if (current.index >= N * M) continue;

        // // 현재 위치에 벽을 세우지 않는 경우
        // s.push({current.index + 1, current.cnt});

        // // 현재 위치에 벽을 세울 수 있는 경우
        // if (map[current.index / M][current.index % M] == 0) {
        //     map[current.index / M][current.index % M] = 1;
        //     s.push({current.index + 1, current.cnt + 1});
        //     map[current.index / M][current.index % M] = 0;  // 복구
        // }
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