class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        best = 0

        def height(node):
            nonlocal best
            if node is None:
                return 0
            left, right = height(node.left), height(node.right)
            best = max(best, left + right)
            return 1 + max(left, right)

        height(root)
        return best
