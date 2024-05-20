#include <iostream>
#include <utility>
#include <cmath>
#include <vector>
#include <climits>

using namespace std;

int N,M;
vector<int> map;
int total_cnt;

void dfs(int idx)
{
    if (idx == N) return;
    
    for (int i = idx; i < N; i++)
    {
        
        map.push_back(1);

        if (map.size() == M) {
            total_cnt++;
        }

        dfs(i + 1);
        map.pop_back();
    }
}

void dfs2(int idx)
{
    if (map.size() == M) {
        total_cnt++;
        return;
    }
    
    for (int i = idx; i < N; i++)
    {
        
        map.push_back(1);
        dfs(i + 1);
        map.pop_back();
    }
}



int main()
{
    cin >> N >> M;

    //for (int i = 0; i < N; i++) map.push_back(1);
    dfs(0);
    cout << total_cnt << endl;
    
    return 0;
}