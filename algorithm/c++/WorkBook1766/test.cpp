#include <iostream>
#include <algorithm>
#include <vector>
#include <queue>

using namespace std;

int main()
{
    priority_queue<int> pri_que;
    
    pri_que.push(-1);
    pri_que.push(-12);
    pri_que.push(-5);
    pri_que.push(-50);


    while(!pri_que.empty())
    {
        int result = pri_que.top();
        pri_que.pop();

        cout << result << endl;
    }
    
    

    return 0;
}