#include <iostream>
#include <vector>

using namespace std;

int N,M;
int case_cnt;
vector<int> arrays;

void dfs(int idx, int sum_res)
{
    if (sum_res > M) return;
    if (sum_res == M)
    {
        case_cnt++;
        return;
    }

    if (idx > N) return;

    dfs(idx + 1, sum_res + arrays[idx]);
    
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

    for (int idx = 0; idx < N; idx++)
    {
        dfs(idx, 0);
    }

    cout << case_cnt << endl;

    return 0;
}