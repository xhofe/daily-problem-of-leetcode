// https://leetcode.cn/problems/my-calendar-ii/

struct MyCalendarTwo {
    booked: Vec<(i32, i32)>,
    overlaps: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    fn new() -> Self {
        Self { booked: vec![], overlaps: vec![] }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for &p in self.overlaps.iter() {
            if start < p.1 && end > p.0 {
                return false;
            }
        }
        for &p in self.booked.iter() {
            if start < p.1 && end > p.0 {
                self.overlaps.push((start.max(p.0), end.min(p.1)));
            }
        }
        self.booked.push((start, end));
        true
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */
