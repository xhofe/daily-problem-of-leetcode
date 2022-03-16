// https://leetcode-cn.com/problems/all-oone-data-structure/

type Node struct {
	prev  *Node
	next  *Node
	count int
	str   string
}
type AllOne struct {
	head *Node
	tail *Node
	m    map[string]*Node
}

func Constructor() AllOne {
	return AllOne{m: make(map[string]*Node)}
}

func (this *AllOne) Inc(key string) {
	node, ok := this.m[key]
	if ok {
		// 如果存在，加1并向后移动node
		node.count++
		if node.next != nil && node.count > node.next.count {
			next := node.next
			// 这里的复杂度并不是O(1) 正确的做法应该是将相同count的string用一个set存到一个节点中，就不用查找了
			for next.next != nil && next.next.count == next.count {
				next = next.next
			}
			node.str, next.str = next.str, node.str
			node.count, next.count = next.count, node.count
			this.m[node.str] = node
			this.m[next.str] = next
		}
	} else {
		// 不存在 创建 并放到头部
		node = &Node{count: 1, str: key}
		this.m[key] = node
		if this.head == nil {
			this.head = node
			this.tail = node
		} else {
			head := this.head
			this.head = node
			node.next = head
			head.prev = node
		}
	}
}

// 1 - 2 - 3
func (this *AllOne) Dec(key string) {
	node, _ := this.m[key]
	node.count--
	if node.count == 0 {
		// 移除
		delete(this.m, key)
		prev, next := node.prev, node.next
		if this.head == this.tail {
			this.head = nil
			this.tail = nil
		} else {
			if prev != nil {
				prev.next = next
			} else {
				this.head = next
			}
			if next != nil {
				next.prev = prev
			} else {
				this.tail = prev
			}
		}
	} else {
		// 移动
		if node.prev != nil && node.prev.count > node.count {
			prev := node.prev
			// 同上 复杂度非O(1)
            for prev.prev != nil && prev.prev.count == prev.count {
				prev = prev.prev
			}
			prev.str, node.str = node.str, prev.str
			prev.count, node.count = node.count, prev.count
			this.m[node.str] = node
			this.m[prev.str] = prev
		}
	}
}

func (this *AllOne) GetMaxKey() string {
	if this.tail == nil {
		return ""
	}
	return this.tail.str
}

func (this *AllOne) GetMinKey() string {
	if this.head == nil {
		return ""
	}
	return this.head.str
}

/**
 * Your AllOne object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Inc(key);
 * obj.Dec(key);
 * param_3 := obj.GetMaxKey();
 * param_4 := obj.GetMinKey();
 */