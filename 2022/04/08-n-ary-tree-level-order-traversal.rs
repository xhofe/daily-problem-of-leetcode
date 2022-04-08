// https://leetcode-cn.com/problems/n-ary-tree-level-order-traversal/

/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Children []*Node
 * }
 */

 func levelOrder(root *Node) [][]int {
    queue := list.New()
	queue.PushBack(root)
	res := make([][]int, 0)
    if root == nil {
		return res
	}
	for queue.Len() != 0 {
		l := queue.Len()
		floor := make([]int, l)
		for i := 0; i < l; i++ {
			cur := queue.Front()
			queue.Remove(cur)
			node := cur.Value.(*Node)
			floor[i] = node.Val
			for j := 0; j < len(node.Children); j++ {
				queue.PushBack(node.Children[j])
			}
		}
		res = append(res, floor)
	}
	return res
}