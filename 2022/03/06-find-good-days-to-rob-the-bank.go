// https://leetcode-cn.com/problems/find-good-days-to-rob-the-bank/submissions/

func goodDaysToRobBank(security []int, time int) []int {
    left := make([]int, len(security))
	right := make([]int, len(security))
	tmp := 0
	for i := 1; i < len(security); i++ {
		if security[i] <= security[i-1] {
			tmp++
		} else {
			tmp = 0
		}
		left[i] = tmp
	}
	tmp = 0
	for i := len(security) - 2; i >= 0; i-- {
		if security[i] <= security[i+1] {
			tmp++
		} else {
			tmp = 0
		}
		right[i] = tmp
	}
	res := make([]int, 0)
	for i := 0; i < len(security); i++ {
		if left[i] >= time && right[i] >= time {
			res = append(res, i)
		}
	}
	return res
}