#include <iostream>
#include <vector>
#include <queue>

using namespace std;

int N;
vector<int> adj[10001];
int degree[10001];
int times[10001];
int results[10001];
queue<int> que;
int min_time;

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);

    cin >> N;

    for (int i = 1; i <= N; i++) 
    {
        int use_time, pre_cnt;
        cin >> use_time >> pre_cnt;
        
        times[i] = use_time;    
        results[i] = use_time;

        for (int j = 0; j < pre_cnt; j++)
        {
            int pre;
            cin >> pre;

            adj[pre].push_back(i);
            degree[i]++;
        }
    }
    
    for (int i = 1; i <= N; i++)
    {
        if (degree[i] == 0) que.push(i);
    }
    
    while (!que.empty())
    {
        int cur = que.front();
        que.pop();

        for (int i = 0; i < adj[cur].size(); i++)
        {
            int next = adj[cur][i];
            degree[next]--;
            
            results[next] = max(results[next], results[cur] + times[next]);

            if (degree[next] == 0) que.push(next);
        }
    }

    for (int i = 1; i <= N; i++)
    {
        min_time = max(min_time, results[i]);
    }

    cout << min_time << endl;


    return 0;
}