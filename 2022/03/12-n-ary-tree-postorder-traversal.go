// https://leetcode-cn.com/problems/n-ary-tree-postorder-traversal/

/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Children []*Node
 * }
 */

 func postorder(root *Node) []int {
	res := make([]int, 0)
	var dfs func(node *Node)
	dfs = func(node *Node) {
		if node == nil {
			return
		}
		for _, child := range node.Children {
			dfs(child)
		}
		res = append(res, node.Val)
	}
	dfs(root)
	return res
}