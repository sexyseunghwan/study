#include <iostream>
#include <algorithm>
#include <vector>
#include <string>

using namespace std;

int L,C;
vector<char> voca;
vector<char> char_arr;


void dfs(int idx, int size)
{
    if (size == L) 
    {
        int vowel_cnt = 0;
        int non_vowel_cnt = 0;
        
        for (int i = 0; i < voca.size(); i++) 
        {
            if (voca[i] == 'a' || voca[i] == 'e' || voca[i] == 'i' || voca[i] == 'o' || voca[i] == 'u') ++vowel_cnt;
            else ++non_vowel_cnt;
        }

        if (vowel_cnt >= 1 && non_vowel_cnt >= 2) {

            for (int i = 0; i < voca.size(); i++) 
            {
                cout << voca[i];
            }
            cout << "\n";
        }   

        return;
    }    
    
    for (int i = idx; i < C; i++) 
    {
        voca.push_back(char_arr[i]);
        dfs(i + 1, size + 1);
        voca.pop_back();
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
        char_arr.push_back(input);
    }

    sort(char_arr.begin(), char_arr.end());

    dfs(0,0);   

    return 0;
}