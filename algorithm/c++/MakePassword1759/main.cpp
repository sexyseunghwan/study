#include <iostream>
#include <algorithm>
#include <vector>
#include <string>

using namespace std;


int L,C;
int vow_cnt;
int not_vow_cnt;
vector<char> voca;
vector<char> res;

bool is_vowel(char &input)
{
    char vowels[] = {'a', 'e', 'i', 'o', 'u'};

    for (char vowel : vowels) {
        if (input == vowel) {
            return true; 
        }
    }
    
    return false; 
}

void dfs(int idx)
{
    if (res.size() == L && vow_cnt >= 1 && not_vow_cnt >= 2) 
    {
        for (auto &elem : res)
        {
            cout << elem;
        }

        cout << endl;
        
        return;
    }
    
    for (int i = idx; i < C; i++)
    {
        char selected = voca[i];
        bool vowel_yn = is_vowel(selected);
        
        if (vowel_yn) vow_cnt++;
        else not_vow_cnt++;

        res.push_back(selected);
        dfs(i + 1);
        res.pop_back();

        if (vowel_yn) vow_cnt--;
        else not_vow_cnt--;
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