// https://leetcode.cn/problems/my-calendar-i/

struct MyCalendar {
    map: std::collections::BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self { map: std::collections::BTreeMap::new() }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let a = self.map.range(start..).next();
        if let Some((&k, &v)) = a {
            if k < end {
                return false;
            }
        }
        let b = self.map.range(..start+1).last();
        if let Some((&k, &v)) = b {
            if v > start {
                return false;
            }
        }
        self.map.insert(start, end);
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
