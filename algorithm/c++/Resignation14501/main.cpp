#include <iostream>
#include <cmath>

using namespace std;

int N;
pair<int, int> map[15];
int dp[15];
int max_price;

void dynamic()
{
    for (int i = 0; i <= N; i++)
    {
        dp[i] = max(dp[i], max_price);

        if (i + map[i].first <= N) 
        {
            dp[i + map[i].first] = max(dp[i] + map[i].second, dp[i + map[i].first]);
        }
        
        max_price = max(max_price, dp[i]);
    }
}

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    cin >> N;
    
    for (int i = 0; i < N; i++)
    {
        int d, p;
        cin >> d >> p;
        map[i] = make_pair(d, p);
    }
    
    dynamic();

    cout << max_price << endl;

    return 0;
}


