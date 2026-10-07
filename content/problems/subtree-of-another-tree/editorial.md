## Approach 1: Compare at Every Node

**Intuition.** `subRoot` appears in `root` exactly when one of the nodes of `root` is the top of
an identical tree. So try every node as a starting point and run an "are these two trees the
same?" check from there.

**Algorithm.**

1. `same(a, b)`: both empty means equal; exactly one empty, or different values, means not equal;
   otherwise compare left with left and right with right.
2. `isSubtree(root, subRoot)` is `same(root, subRoot)`, or `isSubtree` on either child of `root`.

```python
class Solution:
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        def same(a, b):
            if a is None or b is None:
                return a is b
            return a.val == b.val and same(a.left, b.left) and same(a.right, b.right)

        if root is None:
            return False
        return same(root, subRoot) or self.isSubtree(root.left, subRoot) or self.isSubtree(root.right, subRoot)
```

- **Time:** `O(m * n)` where `m` and `n` are the sizes of `root` and `subRoot`: each of the `m`
  starting points may compare up to `n` nodes before finding a difference (think of long runs of
  equal values).
- **Space:** `O(h)` for the recursion, where `h` is the height of `root`.

## Approach 2: Serialize and Search for a Substring

**Intuition.** A pre-order listing with explicit markers for missing children identifies a tree
uniquely, and the listing of any subtree is a contiguous piece of the listing of the whole tree.
So `subRoot` is a subtree of `root` exactly when its listing occurs inside the listing of `root`.

Two details make this correct:

- Write a marker (say `#`) for every missing child, otherwise `[1,2]` and `[1,null,2]` collide.
- Put a delimiter *before* every token, otherwise the listing of `[2]` would be found inside the
  listing of `[12]`.

```python
class Solution:
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        def listing(node, out):
            if node is None:
                out.append(",#")
                return out
            out.append("," + str(node.val))
            listing(node.left, out)
            listing(node.right, out)
            return out

        return "".join(listing(subRoot, [])) in "".join(listing(root, []))
```

- **Time:** `O(m + n)` to build the strings; the substring test is linear with an algorithm such
  as KMP (Python's built-in search is fast in practice).
- **Space:** `O(m + n)` for the strings.

## Approach 3: Canonical Subtree Ids (Optimal)

**Intuition.** Give every distinct subtree a small integer id, so that two subtrees get the same
id exactly when they are identical. An empty tree gets id `0`, and a node's subtree is fully
described by the triple `(id of left subtree, value, id of right subtree)`. Storing those
triples in a dictionary hands out ids consistently across both trees, so checking whether
`subRoot` occurs in `root` reduces to checking whether some node of `root` gets `subRoot`'s id.
Because the dictionary keys are the exact triples, there are no hash collisions to worry about.

**Algorithm.**

1. Keep a dictionary `ids` from triple to id.
2. Compute the id of `subRoot` with a post-order walk: `id(node) = ids.setdefault((id(left),
   node.val, id(right)), len(ids) + 1)`.
3. Walk `root` the same way, using the same dictionary, and report whether any node's id equals
   `subRoot`'s.

```python
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
```

- **Time:** `O(m + n)` expected — one dictionary operation per node.
- **Space:** `O(m + n)` for the dictionary, plus `O(h)` for the recursion.
