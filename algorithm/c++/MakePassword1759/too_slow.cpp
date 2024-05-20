#include <iostream>
#include <algorithm>
#include <vector>
#include <string>

using namespace std;


int L,C;
char *arr;
int vow_cnt;
int not_vow_cnt;
vector<char> res;
vector<string> answer;

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
        string ans = "";
        vector<char> inner_res = res;
        sort(inner_res.begin(), inner_res.end());

        for (auto &elem : inner_res) 
        {
            ans += elem;
        }

        answer.push_back(ans);

        return;
    }
    
    for (int i = idx; i < C; i++)
    {
        char selected = arr[i];
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
    
    arr = new char[L];

    for (int i = 0; i < C; i++) cin >> arr[i];
    
    sort(arr, arr + L);
    
    dfs(0);
    
    //sort(answer.begin(), answer.end());

    // for (auto &elem : answer)
    // {
    //     cout << elem << endl;
    // }

    return 0;
}