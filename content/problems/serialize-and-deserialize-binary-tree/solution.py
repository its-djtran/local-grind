class Codec:
    def serialize(self, root: Optional[TreeNode]) -> str:
        out, queue = [], deque([root])
        while queue:
            node = queue.popleft()
            if node is None:
                out.append("#")
            else:
                out.append(str(node.val))
                queue.append(node.left)
                queue.append(node.right)
        return ",".join(out)

    def deserialize(self, data: str) -> Optional[TreeNode]:
        tokens = data.split(",")
        if not data or tokens[0] == "#":
            return None
        root = TreeNode(int(tokens[0]))
        queue, i = deque([root]), 1
        while queue:
            node = queue.popleft()
            if tokens[i] != "#":
                node.left = TreeNode(int(tokens[i]))
                queue.append(node.left)
            if tokens[i + 1] != "#":
                node.right = TreeNode(int(tokens[i + 1]))
                queue.append(node.right)
            i += 2
        return root
