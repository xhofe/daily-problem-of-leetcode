// https://leetcode-cn.com/problems/maximum-difference-between-increasing-elements/

class Solution {
public:
    int maximumDifference(vector<int>& nums) {
        auto min = nums[0];
        auto res = -1;
        for (auto i = 1; i < nums.size(); i++) {
            if (nums[i]>min){
                res = max(res,nums[i]-min);
            }
            if (nums[i]<min){
                min = nums[i];
            }
        }
        return res;
    }
};