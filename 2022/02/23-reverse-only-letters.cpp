// https://leetcode-cn.com/problems/reverse-only-letters/

class Solution {
public:
    string reverseOnlyLetters(string s) {
        int i = 0, j = s.length()-1;
        while (i<j){
            if (isalpha(s[i])&& isalpha(s[j])){
                auto tmp = s[i];
                s[i++] = s[j];
                s[j--] = tmp;
                continue;
            }
            if (!isalpha(s[i]))i++;
            if (!isalpha(s[j]))j--;
        }
        return s;
    }
};