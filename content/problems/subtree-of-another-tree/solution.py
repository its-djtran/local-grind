class Solution:
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        ids = {}
        found = []

        def ident(node, target):
            if node is None:
                return 0
            key = (ident(node.left, target), node.val, ident(node.right, target))
            i = ids.setdefault(key, len(ids) + 1)
            if i == target:
                found.append(node)
            return i

        target = ident(subRoot, -1)
        ident(root, target)
        return bool(found)
