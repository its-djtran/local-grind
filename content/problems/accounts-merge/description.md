You are given a list `accounts`. Each entry `accounts[i]` is a list of strings whose first element `accounts[i][0]` is a person's **name** and whose remaining elements are **email addresses** belonging to that account.

Two accounts belong to the same person exactly when they have at least one email address in common, and this is transitive: if account A shares an email with B, and B shares one with C, then A, B and C all belong to one person, even if A and C share nothing directly. All accounts of one person carry the same name. Different people may have the same name, so a matching name alone does **not** mean two accounts should be merged.

Merge the accounts and return one entry per person in the form `[name, email_1, email_2, ...]`: the name first, followed by every distinct email of that person exactly once, **sorted in increasing ASCII order**. The people themselves may be listed in any order.

<!-- examples -->

**Constraints:**

- `1 <= accounts.length <= 1000`
- `2 <= accounts[i].length <= 10`
- `1 <= accounts[i][j].length <= 30`
- `accounts[i][0]` consists of English letters.
- `accounts[i][j]` for `j > 0` is a valid email address. The same email may appear in several accounts, or more than once within one account.
