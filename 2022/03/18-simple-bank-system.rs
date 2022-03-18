// https://leetcode-cn.com/problems/simple-bank-system/

struct Bank {
    balance: Vec<i64>,
}

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        return Bank { balance };
    }

    fn is_valid(&self, account: i32) -> bool {
        if account >= 1 && account <= self.balance.len() as i32 {
            true
        } else { false }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if !self.is_valid(account1) || !self.is_valid(account2) { return false; }
        if self.balance[account1 as usize - 1] >= money {
            self.balance[account1 as usize - 1] -= money;
            self.balance[account2 as usize - 1] += money;
            true
        } else {
            false
        }
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if !self.is_valid(account) { return false; }
        self.balance[account as usize - 1] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if !self.is_valid(account) { return false; }
        if self.balance[account as usize - 1] >= money {
            self.balance[account as usize - 1] -= money;
            true
        } else { false }
    }
}