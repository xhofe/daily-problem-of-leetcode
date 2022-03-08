// https://leetcode-cn.com/problems/where-will-the-ball-fall/

class Solution {
public:
    vector<int> findBall(vector<vector<int>>& grid) {
        vector<int> states(grid[0].size());
        for (int i = 0; i < grid[0].size(); ++i) {
            states[i] = i;
        }
        for (auto & i : grid) {
            for (int j = 0; j < states.size(); ++j) {
                if (states[j]==-1)continue;
                if (i[states[j]]==1){
                    if (states[j]==states.size()-1||i[states[j]+1]==-1){
                        states[j]=-1;
                    } else{
                        states[j]++;
                    }
                } else{
                    if (states[j]==0||i[states[j]-1]==1){
                        states[j]=-1;
                    } else{
                        states[j]--;
                    }
                }
            }
        }
        return states;
    }
};