#include <iostream>

using namespace std;

int N, S;
int num_matrix[20];
int visited_matrix[20];
int sum_match;

void dfs(int idx, bool correct, int sum);

int main() 
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);

    cin >> N >> S;
    
    for (int i = 0; i < N; i++) cin >> num_matrix[i];

    dfs(0, true, 0);
    dfs(0, false, 0);

    cout << sum_match << endl;

    return 0;
}

void dfs(int idx, bool correct, int sum) 
{
    if (idx == N) return;
    
    if (correct) {
        sum += num_matrix[idx];
        if (sum == S) sum_match++;
    }

    idx++;

    dfs(idx, true, sum);
    dfs(idx, false, sum);
}