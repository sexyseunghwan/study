#include <iostream>
#include <algorithm>
#include <vector>

using namespace std;

int L,C;
vector<char> voca;
vector<char> pass;
int vowel_cnt;
int non_vowel_cnt;

void dfs(int idx)
{
    
    if (pass.size() == L)
    {
        vowel_cnt = 0;
        non_vowel_cnt = 0;

        for (auto &elem : pass)
        {
            if (elem == 'a' || elem == 'e' || elem == 'i' || elem == 'o' || elem == 'u') vowel_cnt++;
            else non_vowel_cnt++;
        }
        
        if (non_vowel_cnt >= 2 && vowel_cnt >= 1) 
        {
            for (auto &elem : pass) cout << elem;
            cout << endl;
        }
        
        return;
    }

    for (int i = idx; i < C; i++)
    {
        pass.push_back(voca[i]);
        dfs(i+1);
        pass.pop_back();
    }
}

int main()
{
    ios_base::sync_with_stdio(0);
    cin.tie(0), cout.tie(0);

    cin >> L >> C;

    for (int i = 0; i < C; i++)
    {
        char input;
        cin >> input;
        voca.push_back(input);
    }

    sort(voca.begin(), voca.end());

    dfs(0);

    return 0;
}