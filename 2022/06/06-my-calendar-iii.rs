// https://leetcode.cn/problems/my-calendar-iii/

struct MyCalendarThree {
    map: std::collections::BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        Self {
            map: std::collections::BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.map.entry(start).or_default() += 1;
        *self.map.entry(end).or_default() -= 1;
        let mut count = 0;
        let mut ans = 0;
        for &v in self.map.values() {
            count += v;
            ans = ans.max(count);
        }
        ans
    }
}
/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(start, end);
 */
