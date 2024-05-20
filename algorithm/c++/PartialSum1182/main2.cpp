#include <iostream>
#include <vector>

using namespace std;

int N,S;
vector<int> arr;
int total_cnt;

void dfs(int idx, int sum)
{
    if (idx == N) return;
    
    for (int i = idx; i < N; i++)
    {
        int partial_sum = sum + arr[i];
        
        if (partial_sum == S) total_cnt++;

        dfs(i + 1, partial_sum);
    }
}

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    cin >> N >> S;    

    for (int i = 0; i < N; i++)
    {
        int input;
        cin >> input;

        arr.push_back(input);
    }

    dfs(0,0);

    cout << total_cnt << endl;

    return 0;
}