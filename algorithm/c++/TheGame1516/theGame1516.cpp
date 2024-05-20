#include <iostream>
#include <algorithm>
#include <string>
#include <vector>
#include <queue>

using namespace std;

int N;
vector<int> ver[501];
vector<int> adj[501];
queue<int> que;

int times[501];
int result[501];
int in_deg[501];

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    cin >> N;
    
    for(int i = 1; i <= N; i++)
    {
        int input;
        cin >> input;

        while(input != -1)
        {
            ver[i].push_back(input);
            cin >> input;
        }
    }

    for (int i = 1; i <= N; i++)
    {
        times[i] = ver[i][0];
    }

    for (int i = 1; i <= N; i++)
    {
        for (int j = 1; j < ver[i].size(); j++)
        {
            adj[ver[i][j]].push_back(i);
            in_deg[i]++;
        }
    }

    for (int i = 1; i <= N; i++)
    {
        if (in_deg[i] == 0) que.push(i);
        result[i] = times[i];
    }

    
    while(!que.empty())
    {
        int cur = que.front();
        que.pop();

        for (int i = 0; i < adj[cur].size(); i++)
        {
            int next = adj[cur][i];
            in_deg[next]--;

            result[next] = max(result[next], result[cur] + times[next]);

            if (in_deg[next] == 0) que.push(next);
        }
    }

    for (int i = 1; i <= N; i++) cout << result[i] << endl;


    return 0;
}

