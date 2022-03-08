// https://leetcode-cn.com/problems/sum-of-unique-elements/

func sumOfUnique(nums []int) int {
    m := make(map[int]int)
    for _, num := range nums {
        c,ok := m[num]
        if ok {
            m[num] = c + 1
        }else{
            m[num] = 1
        }
    }
    ans := 0
    for k,v := range m {
        if v == 1 {
            ans += k
        }
    }
    return ans
}