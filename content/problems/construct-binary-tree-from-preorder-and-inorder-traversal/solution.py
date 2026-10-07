class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        pos = {v: i for i, v in enumerate(inorder)}
        nxt = iter(preorder)

        def build(lo, hi):
            if lo > hi:
                return None
            node = TreeNode(next(nxt))
            k = pos[node.val]
            node.left = build(lo, k - 1)
            node.right = build(k + 1, hi)
            return node

        return build(0, len(inorder) - 1)
