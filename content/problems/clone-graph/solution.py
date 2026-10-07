class Solution:
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:
        if node is None:
            return None
        copies = {node: Node(node.val)}
        queue = deque([node])
        while queue:
            cur = queue.popleft()
            for n in cur.neighbors:
                if n not in copies:
                    copies[n] = Node(n.val)
                    queue.append(n)
                copies[cur].neighbors.append(copies[n])
        return copies[node]
