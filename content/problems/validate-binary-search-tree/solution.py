class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        stack, node, prev = [], root, -math.inf
        while stack or node:
            while node:
                stack.append(node)
                node = node.left
            node = stack.pop()
            if node.val <= prev:
                return False
            prev = node.val
            node = node.right
        return True
