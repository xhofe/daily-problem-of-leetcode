// https://leetcode.cn/problems/successor-lcci/

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
 func inorderSuccessor(root *TreeNode, p *TreeNode) *TreeNode {
    if root == nil {
		return nil
	}
	if root.Val <= p.Val {
		return inorderSuccessor(root.Right, p)
	}
	left := inorderSuccessor(root.Left, p)
	if left != nil {
		return left
	}
	return root
}
