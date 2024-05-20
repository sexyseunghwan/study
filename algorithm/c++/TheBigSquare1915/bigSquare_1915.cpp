#include <iostream>
#include <algorithm>
#include <string>

using namespace std;

int N,M;
int map[1010][1010];

int main()
{
    cin >> N >> M;

    for (int i = 1; i <= N; i++)
    {
        string inputs;
        cin >> inputs;

        for (int j = 0; j < M; j++)
        {
            map[i][j+1] = inputs[j] - '0';
        }
    }

    int max_len = 0;

    for (int i = 1; i <= N; i++)
    {   
        for (int j = 1; j <= M; j++)
        {
            if (map[i][j] != 0)
            {
                map[i][j] = min(map[i-1][j-1], min(map[i-1][j], map[i][j-1])) + 1;
                max_len = max(max_len, map[i][j]);
            }
        }
    }

    cout << max_len * max_len << endl;

    return 0;
}