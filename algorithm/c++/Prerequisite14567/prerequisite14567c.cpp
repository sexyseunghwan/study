#include <iostream>
#include <algorithm>
#include <vector>
#include <queue>

using namespace std;

int N,M;
vector<int> adj[1001];
int degree[1001];
int result_list[1001];

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);

    cin >> N >> M;
    
    for (int i = 0; i < M; i++)
    {
        int pre, post;
        cin >> pre >> post;

        adj[pre].push_back(post);
        degree[post]++;
    }
    
    queue<int> que;

    for (int i = 1; i <= N; i++)
    {
        result_list[i] = 1;
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

            result_list[next] = max(result_list[next], result_list[cur] + 1);

            if (degree[next] == 0) que.push(next);
        }
    }
    
    for (int i = 1; i <= N; i++) cout << result_list[i] << " ";
    
    return 0;
}