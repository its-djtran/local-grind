## Approach 1: Recursion

**Intuition.** If either list is empty, the answer is the other list. Otherwise the smaller of
the two heads must come first, and what follows it is the merge of everything else, which is the
same problem on a shorter input.

```python
class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        if list1 is None:
            return list2
        if list2 is None:
            return list1
        if list1.val <= list2.val:
            list1.next = self.mergeTwoLists(list1.next, list2)
            return list1
        list2.next = self.mergeTwoLists(list1, list2.next)
        return list2
```

- **Time:** `O(n + m)` — one call per node.
- **Space:** `O(n + m)` for the call stack.

## Approach 2: Iterative Merge with a Dummy Head (Optimal)

**Intuition.** Build the result left to right with a `tail` pointer. A placeholder `dummy` node
sits in front of the result, so appending the first real node is no different from appending any
other.

**Algorithm.**

1. Create `dummy` and set `tail = dummy`.
2. While both lists are non-empty, detach the head with the smaller value (prefer `list1` on
   ties), link it after `tail`, and advance `tail`.
3. One list is now empty. Link the other one (possibly empty) after `tail` in a single step.
4. Return `dummy.next`.

```python
class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        dummy = tail = ListNode()
        while list1 and list2:
            if list1.val <= list2.val:
                tail.next, list1 = list1, list1.next
            else:
                tail.next, list2 = list2, list2.next
            tail = tail.next
        tail.next = list1 or list2
        return dummy.next
```

- **Time:** `O(n + m)`.
- **Space:** `O(1)` — the existing nodes are relinked in place.
