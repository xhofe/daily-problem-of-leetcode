// https://leetcode-cn.com/problems/construct-quad-tree/submissions/

/**
 * Definition for a QuadTree node.
 * type Node struct {
 *     Val bool
 *     IsLeaf bool
 *     TopLeft *Node
 *     TopRight *Node
 *     BottomLeft *Node
 *     BottomRight *Node
 * }
 */

func val(grid [][]int, t, b, l, r int) int {
	v := -1
	for i := t; i < b; i++ {
		for j := l; j < r; j++ {
			if v == -1 {
				v = grid[i][j]
			}
			if v != grid[i][j] {
				return -1
			}
		}
	}
	return v
}

func dfs(grid [][]int, t, b, l, r int) *Node {
	v := val(grid, t, b, l, r)
	node := Node{}
	if v != -1 {
		node.IsLeaf = true
		if v == 1 {
			node.Val = true
		}
		return &node
	}
	node.TopLeft = dfs(grid, t, t+(b-t)/2, l, l+(r-l)/2)
	node.TopRight = dfs(grid, t, t+(b-t)/2, l+(r-l)/2, r)
	node.BottomLeft = dfs(grid, t+(b-t)/2, b, l, l+(r-l)/2)
	node.BottomRight = dfs(grid, t+(b-t)/2, b, l+(r-l)/2, r)
	return &node
}

func construct(grid [][]int) *Node {
	return dfs(grid, 0, len(grid), 0, len(grid))
}
