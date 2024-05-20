#include <iostream>
#include <utility>

using namespace std;

int N;
int T[15], P[15];
int max_cnt = 0;

void dfs(int idx, int price_sum) 
{    
    if (idx > N) return;

    max_cnt = max(max_cnt, price_sum);

    for (int i = idx; i < N; i++) 
    {
        int n_price_sum = P[i] + price_sum;
        dfs(i + T[i], n_price_sum);
    }
}

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);

    cin >> N;

    for (int i = 0; i < N; i++)
    {
        cin >> T[i] >> P[i];
    }

    dfs(0,0);

    cout << max_cnt << endl;

    return 0;
}


//cout << idx << " : " << n_price_sum << endl;
// for (int i = 0; i < N; i++) 
// {
//     cout << T[i] << " : " << P[i] << endl;
// }