#include <iostream>
#include <utility>
#include <cmath>
#include <vector>
#include <climits>

using namespace std;

int N,M;
vector<pair<int, int>> home_vec, chicken_vec, compare_chicken_vec;
int c_dist = INT_MAX;

int distance();
void dfs(int idx);

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    cin >> N >> M;

    for (int i = 0; i < N*N; i++)
    {
        int input;
        cin >> input;

        if (input == 1) home_vec.push_back(make_pair(i / N, i % N));
        if (input == 2) chicken_vec.push_back(make_pair(i / N, i % N));
    }

    dfs(0);

    cout << c_dist << endl;

    return 0;
}


int distance()
{
    int sum_dist = 0;

    for (auto &home : home_vec)
    {
        int min_dist = INT_MAX;

        for (auto &chick : compare_chicken_vec)
        {
            int dist = abs(home.first - chick.first) +  abs(home.second - chick.second);
            min_dist = min(min_dist , dist);
        }

        sum_dist += min_dist;
    }
    
    return sum_dist;
}


// void dfs(int idx)
// {

//     if (compare_chicken_vec.size() == M)
//     {
//         int res_dist = distance();
//         c_dist = min(c_dist , res_dist);
//         return;
//     }


//     for (int i = idx; i < chicken_vec.size(); i++)
//     {
//         compare_chicken_vec.push_back(chicken_vec[i]);
//         dfs(i + 1);
//         compare_chicken_vec.pop_back();
//     }

// }


// 오답.
void dfs(int idx)
{
    if (idx == N) return;

    for (int i = idx; i < chicken_vec.size(); i++)
    {
        compare_chicken_vec.push_back(chicken_vec[i]);
        
        if (compare_chicken_vec.size() == M)
        {
            int res_dist = distance();
            c_dist = min(c_dist , res_dist);
        }
        
        dfs(i + 1);
        compare_chicken_vec.pop_back();
    }
}


