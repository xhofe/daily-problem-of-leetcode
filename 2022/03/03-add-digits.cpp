// https://leetcode-cn.com/problems/add-digits/

class Solution {
public:
    int addDigits(int num) {
        auto res = num % 9;
        if(num != 0 && res == 0)
            return 9;
        return res;
    }
};