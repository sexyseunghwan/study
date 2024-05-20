#include <iostream>
#include <algorithm>
#include <vector>
#include <queue>

using namespace std;

int N,M;
vector<int> adj[32001];
queue<int> que;
int degree[32001];

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
        if (degree[i] == 0) 
        {
            que.push(i);
        }
    }

    while(!que.empty())
    {
        int cur = que.front();
        que.pop();

        cout << cur << " "; 

        for (int i = 0; i < adj[cur].size(); i++) 
        {
            int next = adj[cur][i];
            degree[next]--;
            
            if (degree[next] == 0) 
            {
                que.push(next);
            }
        }
    }

    return 0;
}

