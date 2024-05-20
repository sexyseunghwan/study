#include <iostream>
#include <utility>
#include <cmath>
#include <vector>
#include <climits>

using namespace std;

int N,M;
vector<pair<int, int>> chicken_vec, home_vec, selected_chicken;
int min_dist_total = INT_MAX;

int calculate_distance(); 
void dfs(int start_idx);

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);
    
    cin >> N >> M;

    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            int input;
            cin >> input;

            if (input == 1) home_vec.push_back(make_pair(i,j));
            else if (input == 2) chicken_vec.push_back(make_pair(i,j));
        }
    }
    
    dfs(0);

    cout << min_dist_total << endl;

    return 0;
}

int calculate_distance() {

    int add_dist = 0;

    for (auto& home : home_vec) 
    {
        int inner_min_dist = INT_MAX;

        for (auto& chicken : selected_chicken)
        {
            int dx = abs(home.first - chicken.first);
            int dy = abs(home.second - chicken.second);

            inner_min_dist = min(inner_min_dist, dx+ dy);
        }

        add_dist += inner_min_dist;
    }

    return add_dist;
}

void dfs(int start_idx)
{
    if (selected_chicken.size() == M)
    {
        int dist_val = calculate_distance();
        min_dist_total = min(min_dist_total, dist_val);
        return;
    }

    for (int i = start_idx; i < chicken_vec.size(); i++)
    {
        selected_chicken.push_back(make_pair(chicken_vec.at(i).first, chicken_vec.at(i).second));
        dfs(i+1);
        selected_chicken.pop_back();
    }

}