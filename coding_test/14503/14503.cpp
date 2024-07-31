#include <iostream>


using namespace std;

int N,M;
int r_r, r_c, r_v;
int map[50][50];
int dr[4] = {0,0,-1,1};
int dc[4] = {1,-1,0,0};
int vr[4] = {-1,0,1,0};
int vc[4] = {0,1,0,-1};
int clean_cnt;



void robot_clean_up(int r, int c)
{

    if (map[r][c] != 2) 
    {
        map[r][c] = 2;
        clean_cnt++;
    } 
    
    int check_flag = false;
    
    for (int i = 0; i < 4; i++)
    {
        int nr = r + dr[i];
        int nc = c + dc[i];

        if (nr >= 0 && nr < N && nc >= 0 && nc < M && map[nr][nc] == 0)
        {
            check_flag = true;
            break;
        }
    }
    
    if (check_flag) {
        
        for (int i = 0; i < 4; i++)
        {
            r_v = (r_v + 3) % 4;
            int n_r = r + vr[r_v];
            int n_c = c + vc[r_v];

            if (map[n_r][n_c] == 0)
            {
                robot_clean_up(n_r, n_c);
                break;
            }
        }

    } else {

        int n_r = r - vr[r_v];
        int n_c = c - vc[r_v];    

        if (map[n_r][n_c] != 1)
        {
            robot_clean_up(n_r, n_c);
        }
    }    
}


int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    cin >> N >> M;
    cin >> r_r >> r_c >> r_v;
    
    for (int i = 0; i < N*M; i++) cin >> map[i/M][i%M];

    robot_clean_up(r_r, r_c);

    cout << clean_cnt << endl;
    
    return 0;
}