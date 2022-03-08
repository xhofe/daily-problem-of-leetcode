// https://leetcode-cn.com/problems/sum-of-subarray-ranges/

class Solution {
public:
    long long subArrayRanges(vector<int>& nums) {
        auto len = nums.size();
        long long sum = 0;
        int mi,ma;
        for (int i = 0; i < len - 1; ++i) {
            mi = nums[i];
            ma = nums[i];
            for (int j = i + 1; j < len; ++j) {
                mi = min(mi,nums[j]);
                ma = max(ma,nums[j]);
                sum += (ma - mi);
            }
        }
        return sum;
    }
};