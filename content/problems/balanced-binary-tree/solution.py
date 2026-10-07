class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        def check(node):
            if node is None:
                return 0
            left = check(node.left)
            if left < 0:
                return -1
            right = check(node.right)
            if right < 0 or abs(left - right) > 1:
                return -1
            return 1 + max(left, right)

        return check(root) >= 0
