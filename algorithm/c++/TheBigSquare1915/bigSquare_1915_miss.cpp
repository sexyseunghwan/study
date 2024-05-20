#include <iostream>
#include <algorithm>
#include <string>

using namespace std;

int N,M;
int map[1000][1000];

int main()
{
    cin >> N >> M;

    for (int i = 0; i < N; i++)
    {
        string inputs;
        cin >> inputs;

        for (int j = 0; j < M; j++)
        {
            map[i][j] = inputs[j] - '0';
        }
    }

    int max_len = 0;

    for (int i = 0; i < N; i++)
    {   
        for (int j = 0; j < M; j++)
        {
            if (map[i][j] != 0)
            {
                int i_m = i - 1;
                int j_m = j - 1;
                
                int map_im_jm = (i_m < 0 || j_m < 0) ? 0 : map[i-1][j-1];
                int map_im_j = (i_m < 0) ? 0 : map[i-1][j];
                int map_i_jm = (j_m < 0) ? 0 : map[i][j-1];

                map[i][j] = min(map_im_jm, min(map_im_j, map_i_jm)) + 1;
                max_len = max(max_len, map[i][j]);
            }
        }
    }
    
    cout << max_len * max_len << endl;

    return 0;
}