#include <iostream>
#include <vector>

using namespace std;

int N,M;
int case_cnt;
vector<int> sequence;


void two_pointer()
{
    
    int top_idx = 0;
    int bottom_idx = 0;
    int sum = 0;

    while(bottom_idx <= N)
    {
        if (sum < M) {
            sum += sequence[bottom_idx];
            bottom_idx++;
        }

        if (sum >= M) {
            if (sum == M) case_cnt++;
            sum -= sequence[top_idx];
            top_idx++;
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

    two_pointer();

    cout << case_cnt << endl;

    return 0;
}