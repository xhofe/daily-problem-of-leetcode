// https://leetcode-cn.com/problems/maximum-number-of-achievable-transfer-requests/

class Solution {
public:
    int res;

    int maximumRequests(int n, vector<vector<int>> &requests) {
        res = 0;
        vector<int> state(n, 0);
        backtracking(0, 0, requests, state);
        return res;
    }

    void backtracking(int i, int num, vector<vector<int>> &requests, vector<int> &state) {
        if (isFeasible(state)) {
            this->res = max(this->res, num);
        }
        if (i >= requests.size()) {
            return;
        }
        // choose
        state[requests[i][0]]--;
        state[requests[i][1]]++;
        backtracking(i + 1, num + 1, requests, state);
        state[requests[i][0]]++;
        state[requests[i][1]]--;
        // not choose
        backtracking(i + 1, num, requests, state);
    }

    static bool isFeasible(vector<int> &state) {
        return all_of(state.cbegin(), state.cend(), [](int i) {
            return i == 0;
        });
    }
};