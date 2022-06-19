// https://leetcode.cn/problems/4ueAj6/

/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Next *Node
 * }
 */

func insert(aNode *Node, x int) *Node {
	if aNode == nil {
		node := Node{
			Val: x,
		}
		node.Next = &node
		return &node
	}
	dummy := aNode
	for aNode.Next != dummy {
		if aNode.Val > aNode.Next.Val {
			if aNode.Val < x || x < aNode.Next.Val {
				break
			}
		}
		if aNode.Val <= x && x <= aNode.Next.Val {
			break
		}
		aNode = aNode.Next
	}
	newNode := Node{Val: x}
	newNode.Next = aNode.Next
	aNode.Next = &newNode
	return dummy
}
