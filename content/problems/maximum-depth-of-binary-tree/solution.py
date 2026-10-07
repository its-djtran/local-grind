class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        depth, level = 0, [root] if root else []
        while level:
            depth += 1
            level = [c for node in level for c in (node.left, node.right) if c]
        return depth
