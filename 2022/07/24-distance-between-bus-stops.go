// https://leetcode.cn/problems/distance-between-bus-stops/

func distanceBetweenBusStops(distance []int, start int, destination int) int {
    sum := 0
	d := 0
	if start > destination {
		start, destination = destination, start
	}
	for i := 0; i < len(distance); i++ {
		sum += distance[i]
		if start<= i && i<destination {
			d += distance[i]
		}
	}
	if sum-d<d {
		return sum-d
	}
	return d
}
