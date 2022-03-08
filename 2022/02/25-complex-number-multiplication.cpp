// https://leetcode-cn.com/problems/complex-number-multiplication/

class Solution{
public:
  string complexNumberMultiply(string num1, string num2)
  {
    int a1, b1, a2, b2;
    auto i1 = num1.find('+');
    a1 = atoi(num1.substr(0, i1).c_str());
    b1 = atoi(num1.substr(i1 + 1).c_str());
    auto i2 = num2.find('+');
    a2 = atoi(num2.substr(0, i2).c_str());
    b2 = atoi(num2.substr(i2 + 1).c_str());
    auto a = a1 * a2 - b1 * b2;
    auto b = a1 * b2 + a2 * b1;
    return to_string(a) + "+" + to_string(b) + "i";
  }
};