#include <iostream>
#include <vector>

using namespace std;

int N,M;
int case_cnt;
vector<int> sequence;

void partial_sum(int start_idx)
{    
    int sum_res = 0;

    for (int i = start_idx; i < N; i++)
    {
        sum_res += sequence[i];

        if (sum_res == M) {
            case_cnt++;
            return;
        }
    }
}

int main()
{

    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);

    cin >> N >> M;

    for (int i = 0; i < N; i++) 
    {
        int input;
        cin >> input;

        sequence.push_back(input);
    }
    
    for (int i = 0; i < sequence.size(); i++)
    {
        partial_sum(i);
    }

    cout << case_cnt << endl;

    return 0;
}