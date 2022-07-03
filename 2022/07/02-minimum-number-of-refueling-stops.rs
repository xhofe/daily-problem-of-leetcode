impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut dist = start_fuel;
        let mut ans = 0;
        let mut i = 0;
        let mut heap = std::collections::BinaryHeap::new();
        while dist < target {
            while i < stations.len() && stations[i][0] <= dist {
                heap.push(stations[i][1]);
                i += 1;
            }
            if heap.is_empty() {
                return -1;
            }
            let fuel = heap.pop().unwrap();
            dist += fuel;
            ans += 1;
        }
        ans
    }
}
