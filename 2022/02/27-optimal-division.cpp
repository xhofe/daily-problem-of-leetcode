// https://leetcode-cn.com/problems/optimal-division/

class Solution {
public:
    string optimalDivision(vector<int>& nums) {
        string s = to_string(nums[0]);
        if (nums.size()==1){
            return s;
        }
        if (nums.size()==2){
            s.append("/");
            s.append(to_string(nums[1]));
            return s;
        }
        s.append("/(");
        for (int i = 1; i < nums.size(); ++i) {
            s.append(to_string(nums[i]));
            if (i!=nums.size()-1){
                s.append("/");
            }
        }
        s.append(")");
        return s;
    }
};