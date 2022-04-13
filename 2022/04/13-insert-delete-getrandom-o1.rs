// https://leetcode-cn.com/problems/insert-delete-getrandom-o1/

use std::collections::{HashMap};

struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        return Self {
            map: HashMap::new(),
            list: vec![],
        };
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) { false } else {
            self.list.push(val);
            self.map.insert(val,self.list.len()-1);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) {
            false
        }else {
            let &index = self.map.get(&val).unwrap();
            let &last = self.list.last().unwrap();
            self.list[index] = last;
            self.list.pop();
            self.map.insert(last, index);
            self.map.remove(&val);
            true
        }
    }

    fn get_random(&self) -> i32 {
        let rand_number = rand::random::<usize>() % self.list.len();
        self.list[rand_number]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */