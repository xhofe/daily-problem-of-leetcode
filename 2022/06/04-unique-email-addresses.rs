// https://leetcode.cn/problems/unique-email-addresses/

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        emails.into_iter().map(|x|{
            let mut email = x.split('@').collect::<Vec<&str>>();
            let mut local = email[0].split('+').collect::<Vec<_>>()[0].to_owned();
            local.retain(|x| x != '.');
            email[0] = local.as_str();
            email.join("@")
        }).collect::<std::collections::HashSet<_>>().len() as i32
    }
}
