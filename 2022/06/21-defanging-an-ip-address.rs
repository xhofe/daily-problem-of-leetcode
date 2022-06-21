// https://leetcode.cn/problems/defanging-an-ip-address/

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace(".","[.]")
    }
}
