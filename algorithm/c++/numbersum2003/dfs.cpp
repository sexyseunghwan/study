#include <iostream>
#include <vector>

using namespace std;

int N,M;
int case_cnt;
vector<int> arrays;

void dfs(int idx, int sum_res, bool flag)
{
    if (sum_res == M)
    {
        case_cnt++;
        return;
    }

    for (int i = idx; i < N; i++)
    {
        dfs(i+1, sum_res + arrays[i], true);
        if (flag) return;
    }
}

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    cin >> N >> M;

    for (int i = 0; i < N; i++)
    {
        int input;
        cin >> input;

        arrays.push_back(input);
    }

    dfs(0,0,false);

    cout << case_cnt << endl;

    return 0;
}