#include <iostream>
#include <algorithm>

using namespace std;

int N,M,c,r,v;
int cleanCnt;
int map[51][51];
int visited[51][51];
int dr[4] = {0,0,1,-1};
int dc[4] = {1,-1,0,0};



int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    int a = -1;

    cout << a % 3 << endl;
    // cin >> N >> M >> c >> r >> v;

    // for (int i = 0; i < N*M; i++) cin >> map[i/N][i%N];
    
    
    
    // //test
    // // for (int i = 0; i < N; i++)
    // // {
    // //     for (int j = 0; j < M; j++) 
    // //     {
    // //         cout << map[i][j] << " ";
    // //     }
    // //     cout << endl;
    // // }
    


    return 0;
}