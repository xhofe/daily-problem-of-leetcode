// https://leetcode-cn.com/problems/n-ary-tree-preorder-traversal/

/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Children []*Node
 * }
 */

func preorder(root *Node) []int {
	res := make([]int, 0)
	dfs(root, &res)
	return res
}

func dfs(node *Node, res *[]int) {
	if node == nil {
		return
	}
	*res = append(*res, node.Val)
	for i := 0; i < len(node.Children); i++ {
		dfs(node.Children[i], res)
	}
}