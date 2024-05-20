#include <iostream>
#include <vector>
#include <queue>

using namespace std;

int N,M;
vector<int> adj[1001];
int degree[1001];
queue<int> que;

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    cin >> N >> M;

    for (int i = 0; i < M; i++)
    {
        int cnt;
        cin >> cnt;

        vector<int> inputs_v;

        for (int j = 0; j < cnt; j++)
        {
            int input;
            cin >> input;
            inputs_v.push_back(input);   
        }
        
        for (int j = 0; j < cnt-1; j++)
        {
            adj[inputs_v[j]].push_back(inputs_v[j+1]);
            degree[inputs_v[j+1]]++;
        }
    }

    for (int i = 1; i <= N; i++) 
    {
        if (degree[i] == 0) que.push(i);
    }

    vector<int> result_vec; 

    while (!que.empty())
    {
        int cur = que.front();
        que.pop();
        result_vec.push_back(cur);

        for (int i = 0; i < adj[cur].size(); i++)
        {
            int next = adj[cur][i];
            degree[next]--;

            if (degree[next] == 0) que.push(next);
        }
    }

    if (result_vec.size() != N) cout << 0 << endl;
    else
    {
        for (int i = 0; i < result_vec.size(); i++) cout << result_vec[i] << endl;
    } 


    return 0;
}