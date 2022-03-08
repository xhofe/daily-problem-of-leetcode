// https://leetcode-cn.com/problems/zigzag-conversion/

class Solution {
public:
    string convert(string s, int numRows) {
        if(numRows==1){
            return s;
        }
        auto strings = vector<string>(numRows);
        int index = 0;
        int flag = 1;
        for (char i : s)
        {
            strings[index] += i;
            index += flag;
            if (index==numRows-1)
            {
                flag = -1;
            }
            if (index==0)
            {
                flag = 1;
            }
        }
        string res;
        for (int i = 0; i < numRows; i++)
        {
            res.append(strings[i]);
        }
        return res;
    }
};