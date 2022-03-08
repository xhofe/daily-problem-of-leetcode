// https://leetcode-cn.com/problems/plates-between-candles/

func platesBetweenCandles(s string, queries [][]int) []int {
	res := make([]int, len(queries))
	pre := make([]int, len(s))
	leftN, rightN := make([]int, len(s)), make([]int, len(s))
	near := -1
	for i, c := range s {
		if i != 0 {
			pre[i] = pre[i-1]
		}
		if c == '*' {
			pre[i]++
		} else {
			near = i
		}
		leftN[i] = near
	}
	near = -1
	for i := len(s) - 1; i >= 0; i-- {
		if s[i] == '|' {
			near = i
		}
		rightN[i] = near
	}
	for i, query := range queries {
		left, right := query[0], query[1]
		l, r := rightN[left], leftN[right]
		if l >= r || l == -1 || r== -1 {
			continue
		}
		res[i] = pre[r] - pre[l]
	}
	return res
}