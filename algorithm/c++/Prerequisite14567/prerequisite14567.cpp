#include <iostream>
#include <vector>
#include <queue>

using namespace std;

int N,M;
vector<int> adj[1001];
int degree[1001];
int result[1001];
queue<int> que;

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

    for (int i = 1; i <= N; i++)
    {
        result[i] = 1;

        if (degree[i] == 0) que.push(i);
    }

    while (!que.empty())
    {
        int cur = que.front();
        que.pop();

        for (int i = 0; i < adj[cur].size(); i++) 
        {
            int next = adj[cur][i];
            result[next] = max(result[next], result[cur] + 1);
            degree[next]--;

            if (degree[next] == 0) que.push(next);
        }
    }
    
    for (int i = 1; i <= N; i++) cout << result[i] << " ";

    return 0;
}