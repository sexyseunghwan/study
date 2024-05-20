#include <iostream>

using namespace std;

int N;
pair<int, int> map[15];
int max_cost;

void dfs(int idx, int sum)
{
    
    if (idx > N) return;

    max_cost = max(max_cost, sum);

    for (int i = idx; i < N; i++)
    {
        dfs(i + map[i].first, sum + map[i].second);
    }
}

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);

    cin >> N;

    for (int i = 0; i < N; i++)
    {
        int d, t;
        cin >> d >> t;
        map[i] = make_pair(d, t);
    }
    
    dfs(0,0);

    cout << max_cost << endl;
    
    return 0;
}

