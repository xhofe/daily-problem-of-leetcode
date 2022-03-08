// https://leetcode-cn.com/problems/pancake-sorting/

func pancakeSort(arr []int) []int {
	res := make([]int, 0)
	n := len(arr)
	for n > 0 {
		i := index(arr, n)
		res = append(res, i+1)
		reverse(arr, 0, i)
		res = append(res, n)
		reverse(arr, 0, n-1)
		n -= 1
	}
	return res
}

func index(arr []int, n int) int {
	for i, i2 := range arr {
		if i2 == n {
			return i
		}
	}
	return 0
}

func reverse(arr []int, i, j int) {
	for i < j {
		arr[i], arr[j] = arr[j], arr[i]
		i++
		j--
	}
}