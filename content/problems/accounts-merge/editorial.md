Model the problem as a graph whose nodes are email addresses: all emails listed in the same
account are connected. Every person corresponds to one **connected component**, and the name of
any account in that component is the person's name.

## Approach 1: Merge Until Nothing Changes

**Intuition.** Keep a list of groups (name plus set of emails), one per account. Whenever two
groups share an email, merge them, and start looking again; stop when no two groups intersect.

```python
class Solution:
    def accountsMerge(self, accounts: List[List[str]]) -> List[List[str]]:
        groups = [(acc[0], set(acc[1:])) for acc in accounts]
        merged = True
        while merged:
            merged = False
            for i in range(len(groups)):
                for j in range(i + 1, len(groups)):
                    if groups[i][1] & groups[j][1]:
                        groups[i][1].update(groups[j][1])
                        groups.pop(j)
                        merged = True
                        break
                if merged:
                    break
        return [[name] + sorted(emails) for name, emails in groups]
```

- **Time:** `O(n^3 * k)` in the worst case for `n` accounts of up to `k` emails: up to `n`
  merges, each found by a quadratic scan of set intersections.
- **Space:** `O(n * k)`.

## Approach 2: Depth-First Search over the Email Graph

**Intuition.** Build the email graph explicitly. Linking each email to the first email of its
account is enough to connect the whole account (a star instead of a clique). Then run a DFS from
each unvisited email to collect its component.

```python
class Solution:
    def accountsMerge(self, accounts: List[List[str]]) -> List[List[str]]:
        graph = defaultdict(list)
        owner = {}
        for name, *emails in accounts:
            first = emails[0]
            for e in emails:
                graph[first].append(e)
                graph[e].append(first)
                owner[e] = name
        seen, result = set(), []
        for e in owner:
            if e in seen:
                continue
            seen.add(e)
            stack, component = [e], []
            while stack:
                cur = stack.pop()
                component.append(cur)
                for nxt in graph[cur]:
                    if nxt not in seen:
                        seen.add(nxt)
                        stack.append(nxt)
            result.append([owner[e]] + sorted(component))
        return result
```

- **Time:** `O(N log N)` where `N = n * k` is the total number of emails: linear-time graph work
  plus sorting each component.
- **Space:** `O(N)`.

## Approach 3: Union-Find over Emails (Optimal)

**Intuition.** Union-find expresses "these emails belong together" directly. For every account,
union each of its emails with the account's first email. Afterwards, emails with the same root
form one person; group them by root, sort, and prepend the owner's name.

**Algorithm.**

1. For each account, record `owner[email] = name` and `union(first_email, email)` for every email.
2. Group all emails by `find(email)`.
3. For each group, output `[owner[root]] + sorted(group)`.

```python
class Solution:
    def accountsMerge(self, accounts: List[List[str]]) -> List[List[str]]:
        parent = {}

        def find(x):
            while parent[x] != x:
                parent[x] = parent[parent[x]]
                x = parent[x]
            return x

        owner = {}
        for name, *emails in accounts:
            for e in emails:
                if e not in parent:
                    parent[e] = e
                    owner[e] = name
            root = find(emails[0])
            for e in emails[1:]:
                other = find(e)
                if other != root:
                    parent[other] = root
        groups = defaultdict(list)
        for e in parent:
            groups[find(e)].append(e)
        return [[owner[root]] + sorted(emails) for root, emails in groups.items()]
```

- **Time:** `O(N log N)` — near-constant-time unions and finds for `N` emails, dominated by
  sorting the groups.
- **Space:** `O(N)` for the parent and owner maps.
