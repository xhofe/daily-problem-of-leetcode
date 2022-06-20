// https://leetcode.cn/problems/range-module/

struct RangeModule {
    map: std::collections::BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModule {
    fn new() -> Self {
        Self {
            map: std::collections::BTreeMap::new(),
        }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        let mut removes = vec![];
        let (mut L, mut R) = (left, right);
        for (&r, &l) in self.map.range(left..) {
            if l > right {
                break;
            }
            removes.push(r);
            L = L.min(l);
            R = R.max(r);
        }
        for r in removes {
            self.map.remove(&r);
        }
        self.map.insert(R, L);
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        let (&r,&l) = self.map.range(left..).next().unwrap_or((&0, &0));
        l <= left && right <= r
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let mut removes = vec![];
        let (mut L, mut R) = (right, left);
        for (&r, &l) in self.map.range(left..) {
            if l > right {
                break;
            }
            removes.push(r);
            L = L.min(l);
            R = R.max(r);
        }
        for r in removes {
            self.map.remove(&r);
        }
        if L < left {
            self.map.insert(left, L);
        }
        if R > right {
            self.map.insert(R, right);
        }
    }
}


/**
 * Your RangeModule object will be instantiated and called as such:
 * let obj = RangeModule::new();
 * obj.add_range(left, right);
 * let ret_2: bool = obj.query_range(left, right);
 * obj.remove_range(left, right);
 */
