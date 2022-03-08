// https://leetcode-cn.com/problems/number-of-rectangles-that-can-form-the-largest-square/

func countGoodRectangles(rectangles [][]int) int {
    count,max :=0,0
    for _,rectangle := range rectangles {
        tmp := min(rectangle[0],rectangle[1])
        if tmp > max {
            max = tmp
            count = 1
        }else if tmp == max {
            count +=1
        }
    }
    return count
}

func min(a,b int) int {
    if a > b {
        return b
    }
    return a
}