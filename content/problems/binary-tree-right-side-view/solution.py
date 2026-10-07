class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        view, level = [], [root] if root else []
        while level:
            view.append(level[-1].val)
            level = [c for node in level for c in (node.left, node.right) if c]
        return view
