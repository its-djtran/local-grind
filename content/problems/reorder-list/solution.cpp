class Solution {
public:
    void reorderList(ListNode* head) {
        // Find the end of the first half.
        ListNode *slow = head, *fast = head;
        while (fast->next && fast->next->next) {
            slow = slow->next;
            fast = fast->next->next;
        }

        // Reverse the second half.
        ListNode *prev = nullptr, *cur = slow->next;
        slow->next = nullptr;
        while (cur) {
            ListNode* next = cur->next;
            cur->next = prev;
            prev = cur;
            cur = next;
        }

        // Interleave the two halves.
        ListNode *first = head, *second = prev;
        while (second) {
            ListNode *firstNext = first->next, *secondNext = second->next;
            first->next = second;
            second->next = firstNext;
            first = firstNext;
            second = secondNext;
        }
    }
};
