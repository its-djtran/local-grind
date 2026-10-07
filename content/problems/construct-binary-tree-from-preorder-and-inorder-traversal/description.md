A binary tree has been flattened in two ways: `preorder` lists its values in pre-order (node, then left subtree, then right subtree), and `inorder` lists them in in-order (left subtree, then node, then right subtree).

All values in the tree are distinct. Rebuild the original tree from the two listings and return its root.

<!-- examples -->

**Constraints:**

- `1 <= preorder.length <= 3000`
- `inorder.length == preorder.length`
- `-3000 <= preorder[i], inorder[i] <= 3000`
- `preorder` and `inorder` contain **distinct** values.
- Every value of `inorder` also appears in `preorder`.
- `preorder` is guaranteed to be the pre-order traversal of the tree.
- `inorder` is guaranteed to be the in-order traversal of the tree.
