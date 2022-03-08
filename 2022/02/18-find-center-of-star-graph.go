// https://leetcode-cn.com/problems/find-center-of-star-graph/

func findCenter(edges [][]int) int {
	a, b, c, d := edges[0][0], edges[0][1], edges[1][0], edges[1][1]
	if a == b || a == c || a == d {
		return a
	}
	if b == c || b == d {
		return b
	}
	return c
}