// https://leetcode.cn/problems/validate-ip-address/

// 投机取巧了属于是
impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        let res = query_ip.parse::<std::net::IpAddr>();
        if res.is_err() {
            return "Neither".to_string();
        }
        let ip = res.unwrap();
        if ip.is_ipv4() {
            return "IPv4".to_string();
        }
        if query_ip.contains("::") {
            "Neither".to_string()
        }else {
            "IPv6".to_string()
        }
    }
}
